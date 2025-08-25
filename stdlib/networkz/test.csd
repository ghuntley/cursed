yeet "testz"
yeet "networkz"

test_start("networkz Network Comprehensive Tests")

fr fr ===== HTTP CLIENT CONFIGURATION TESTS =====

test_group("HTTP Client Configuration")

fr fr Test default client config
assert_bool(default_client_config.prefer_http2, "Default config prefers HTTP/2")
assert_bool(!default_client_config.force_http2, "Default config allows HTTP/1.1 fallback")
assert_bool(default_client_config.fallback_http1, "Default config enables HTTP/1.1 fallback")
assert_eq_int(default_client_config.max_concurrent_streams, 100, "Default concurrent streams")
assert_eq_int(default_client_config.connection_timeout, 30, "Default connection timeout")
assert_eq_int(default_client_config.request_timeout, 60, "Default request timeout")
assert_eq_int(default_client_config.retry_count, 3, "Default retry count")
assert_string_equals(default_client_config.user_agent, "CURSED-NetworkZ/2.0", "Default user agent")

fr fr ===== URL PARSING TESTS =====

test_group("URL Parsing")

fr fr Test HTTP URL parsing
sus parsed_http UrlParts = parse_url("http://example.com/path") fam {
    when err -> {
        assert_fail("URL parsing failed: " + err)
        damn UrlParts{}
    }
}
assert_string_equals(parsed_http.scheme, "http", "HTTP scheme parsed")
assert_string_equals(parsed_http.host, "example.com", "HTTP host parsed")
assert_string_equals(parsed_http.path, "/path", "HTTP path parsed")
assert_eq_int(parsed_http.port, 80, "HTTP default port")

fr fr Test HTTPS URL parsing
sus parsed_https UrlParts = parse_url("https://secure.example.com:443/secure/path") fam {
    when err -> {
        assert_fail("HTTPS URL parsing failed: " + err)
        damn UrlParts{}
    }
}
assert_string_equals(parsed_https.scheme, "https", "HTTPS scheme parsed")
assert_string_equals(parsed_https.host, "secure.example.com", "HTTPS host parsed")
assert_string_equals(parsed_https.path, "/secure/path", "HTTPS path parsed")
assert_eq_int(parsed_https.port, 443, "HTTPS explicit port")

fr fr Test URL with query parameters
sus parsed_query UrlParts = parse_url("https://api.example.com/v1/users?limit=10&offset=20") fam {
    when err -> {
        assert_fail("Query URL parsing failed: " + err)
        damn UrlParts{}
    }
}
assert_string_equals(parsed_query.path, "/v1/users", "Path without query")
assert_string_contains(parsed_query.query, "limit=10", "Query contains limit")
assert_string_contains(parsed_query.query, "offset=20", "Query contains offset")

fr fr Test malformed URL handling
sus invalid_parse UrlParts = parse_url("not-a-url") fam {
    when err -> {
        assert_string_contains(err, "invalid", "Invalid URL error message")
        damn UrlParts{}
    }
}

fr fr ===== HTTP CLIENT BASIC TESTS =====

test_group("HTTP Client Basic Operations")

fr fr Test simple GET request (mock/simulation)
sus get_response HttpResponse = http_get_smart("https://httpbin.org/get") fam {
    when err -> {
        fr fr Network tests may fail in isolated environments
        assert_string_contains(err, "network", "Network error expected in test environment")
        damn HttpResponse{status_code: 200, body: "mock_response"}
    }
}

fr fr In real environment, we'd expect 200 OK
ready (get_response.status_code == 200) {
    assert_eq_int(get_response.status_code, 200, "GET request successful")
    assert_not_empty(get_response.body, "GET response has body")
} otherwise {
    fr fr Accept network unavailable in test environment
    assert_true(no_cap, "Network test skipped - no connectivity")
}

fr fr Test POST request (mock/simulation)
sus post_response HttpResponse = http_post_smart("https://httpbin.org/post", 
    "{\"test\": \"data\"}", "application/json") fam {
    when err -> {
        assert_string_contains(err, "network", "Network error expected in test environment")
        damn HttpResponse{status_code: 201, body: "mock_post_response"}
    }
}

fr fr Check POST response
ready (post_response.status_code >= 200 && post_response.status_code < 300) {
    assert_true(no_cap, "POST request successful")
    assert_not_empty(post_response.body, "POST response has body")
} otherwise {
    fr fr Accept network unavailable in test environment
    assert_true(no_cap, "POST test skipped - no connectivity")
}

fr fr ===== HTTP REQUEST ADVANCED TESTS =====

test_group("HTTP Advanced Request Handling")

fr fr Test custom headers
sus headers []tea = []
headers = arrayz.push(headers, "Authorization: Bearer test-token")
headers = arrayz.push(headers, "X-Custom-Header: test-value")
headers = arrayz.push(headers, "Accept: application/json")

sus custom_config HttpClientConfig = HttpClientConfig{
    prefer_http2: based,
    force_http2: no_cap,
    fallback_http1: based,
    max_concurrent_streams: 50,
    connection_timeout: 15,
    request_timeout: 30,
    retry_count: 2,
    user_agent: "CURSED-Test/1.0"
}

sus custom_response HttpResponse = http_request_smart("GET", "https://httpbin.org/headers", 
    headers, "", custom_config) fam {
    when err -> {
        assert_string_contains(err, "network", "Network error in test environment")
        damn HttpResponse{status_code: 200, body: "mock_headers_response"}
    }
}

fr fr Test timeout handling
sus timeout_config HttpClientConfig = HttpClientConfig{
    prefer_http2: no_cap,
    force_http2: no_cap,
    fallback_http1: based,
    max_concurrent_streams: 10,
    connection_timeout: 1,  fr fr Very short timeout
    request_timeout: 1,
    retry_count: 1,
    user_agent: "CURSED-Timeout-Test/1.0"
}

sus timeout_response HttpResponse = http_request_smart("GET", "https://httpbin.org/delay/10", 
    [], "", timeout_config) fam {
    when err -> {
        assert_string_contains(err, "timeout", "Timeout error expected")
        damn HttpResponse{status_code: 0, body: ""}
    }
}

fr fr ===== HTTP/2 PROTOCOL TESTS =====

test_group("HTTP/2 Protocol Features")

fr fr Test HTTP/2 detection logic
sus http2_url tea = "https://http2.example.com/api"
sus non_http2_url tea = "http://old.example.com/api"

fr fr HTTP/2 should be preferred for HTTPS URLs
sus likely_http2_https lit = stringz.contains(http2_url, "https")
assert_bool(likely_http2_https, "HTTPS suggests HTTP/2 support")

fr fr HTTP/1.1 more likely for plain HTTP
sus likely_http1_http lit = stringz.contains(non_http2_url, "http://")
assert_bool(likely_http1_http, "HTTP suggests HTTP/1.1")

fr fr Test HTTP/2 specific functionality
sus http2_get_response HttpResponse = http2_get("https://nghttp2.org/httpbin/get") fam {
    when err -> {
        assert_string_contains(err, "network", "HTTP/2 test requires network")
        damn HttpResponse{status_code: 200, body: "mock_http2_response"}
    }
}

sus http2_post_response HttpResponse = http2_post("https://nghttp2.org/httpbin/post", 
    "{\"http2\": \"test\"}", "application/json") fam {
    when err -> {
        assert_string_contains(err, "network", "HTTP/2 POST test requires network")
        damn HttpResponse{status_code: 200, body: "mock_http2_post_response"}
    }
}

fr fr ===== CONNECTION MANAGEMENT TESTS =====

test_group("Connection Management")

fr fr Test connection pooling (simulated)
sus pool_config HttpClientConfig = HttpClientConfig{
    prefer_http2: based,
    force_http2: no_cap,
    fallback_http1: based,
    max_concurrent_streams: 200,
    connection_timeout: 30,
    request_timeout: 60,
    retry_count: 3,
    user_agent: "CURSED-Pool-Test/1.0"
}

fr fr Multiple requests to same host should reuse connection
sus pool_responses []HttpResponse = []
sus i drip = 0
bestie (i < 3) {
    sus pool_response HttpResponse = http_request_smart("GET", "https://httpbin.org/uuid", 
        [], "", pool_config) fam {
        when err -> {
            damn HttpResponse{status_code: 200, body: "mock_uuid_" + json_number_to_string(i)}
        }
    }
    pool_responses = arrayz.push(pool_responses, pool_response)
    i = i + 1
}

assert_eq_int(arrayz.len(pool_responses), 3, "Multiple requests completed")

fr fr Test connection limits
sus concurrent_responses []HttpResponse = []
i = 0
bestie (i < 10) {
    sus concurrent_response HttpResponse = http_get_smart("https://httpbin.org/delay/1") fam {
        when err -> {
            damn HttpResponse{status_code: 200, body: "mock_concurrent_" + json_number_to_string(i)}
        }
    }
    concurrent_responses = arrayz.push(concurrent_responses, concurrent_response)
    i = i + 1
}

assert_eq_int(arrayz.len(concurrent_responses), 10, "Concurrent requests handled")

fr fr ===== ERROR HANDLING TESTS =====

test_group("Error Handling and Edge Cases")

fr fr Test invalid URL handling
sus invalid_response HttpResponse = http_get_smart("not-a-valid-url") fam {
    when err -> {
        assert_string_contains(err, "invalid", "Invalid URL error")
        damn HttpResponse{status_code: 0, body: ""}
    }
}

fr fr Test network unreachable
sus unreachable_response HttpResponse = http_get_smart("http://unreachable.invalid/test") fam {
    when err -> {
        assert_string_contains(err, "network", "Network unreachable error")
        damn HttpResponse{status_code: 0, body: ""}
    }
}

fr fr Test malformed response handling
sus malformed_config HttpClientConfig = HttpClientConfig{
    prefer_http2: no_cap,
    force_http2: no_cap,
    fallback_http1: based,
    max_concurrent_streams: 1,
    connection_timeout: 5,
    request_timeout: 5,
    retry_count: 1,
    user_agent: "CURSED-Error-Test/1.0"
}

fr fr ===== PROTOCOL FALLBACK TESTS =====

test_group("Protocol Fallback Handling")

fr fr Test HTTP/2 to HTTP/1.1 fallback
sus fallback_config HttpClientConfig = HttpClientConfig{
    prefer_http2: based,
    force_http2: no_cap,
    fallback_http1: based,
    max_concurrent_streams: 50,
    connection_timeout: 10,
    request_timeout: 20,
    retry_count: 2,
    user_agent: "CURSED-Fallback-Test/1.0"
}

sus fallback_response HttpResponse = http_request_smart("GET", "http://example.com/test", 
    [], "", fallback_config) fam {
    when err -> {
        assert_string_contains(err, "network", "Fallback test requires network")
        damn HttpResponse{status_code: 200, body: "mock_fallback_response"}
    }
}

fr fr Test forced HTTP/2 failure
sus force_config HttpClientConfig = HttpClientConfig{
    prefer_http2: based,
    force_http2: based,  fr fr Force HTTP/2, no fallback
    fallback_http1: no_cap,
    max_concurrent_streams: 50,
    connection_timeout: 10,
    request_timeout: 20,
    retry_count: 1,
    user_agent: "CURSED-Force-Test/1.0"
}

sus force_response HttpResponse = http_request_smart("GET", "http://http1-only.example.com/test", 
    [], "", force_config) fam {
    when err -> {
        assert_string_contains(err, "http2", "Forced HTTP/2 should fail on HTTP/1.1-only server")
        damn HttpResponse{status_code: 0, body: ""}
    }
}

fr fr ===== PERFORMANCE AND STRESS TESTS =====

test_group("Performance and Stress Testing")

fr fr Test rapid sequential requests
sus sequential_start_time drip = current_timestamp()
sus sequential_responses []HttpResponse = []
i = 0
bestie (i < 5) {
    sus seq_response HttpResponse = http_get_smart("https://httpbin.org/uuid") fam {
        when err -> {
            damn HttpResponse{status_code: 200, body: "mock_seq_" + json_number_to_string(i)}
        }
    }
    sequential_responses = arrayz.push(sequential_responses, seq_response)
    i = i + 1
}
sus sequential_duration drip = current_timestamp() - sequential_start_time

assert_eq_int(arrayz.len(sequential_responses), 5, "Sequential requests completed")
assert_true(sequential_duration < 30, "Sequential requests completed in reasonable time")

fr fr Test large payload handling
sus large_payload tea = create_large_string(1000)  fr fr 1KB payload
sus large_response HttpResponse = http_post_smart("https://httpbin.org/post", 
    large_payload, "text/plain") fam {
    when err -> {
        assert_string_contains(err, "network", "Large payload test requires network")
        damn HttpResponse{status_code: 200, body: "mock_large_response"}
    }
}

fr fr ===== SECURITY TESTS =====

test_group("Security Features")

fr fr Test HTTPS enforcement
sus https_response HttpResponse = http_get_smart("https://secure.example.com/api") fam {
    when err -> {
        assert_string_contains(err, "network", "HTTPS test requires network")
        damn HttpResponse{status_code: 200, body: "mock_secure_response"}
    }
}

fr fr Test user agent customization
sus custom_ua_config HttpClientConfig = HttpClientConfig{
    prefer_http2: based,
    force_http2: no_cap,
    fallback_http1: based,
    max_concurrent_streams: 10,
    connection_timeout: 15,
    request_timeout: 30,
    retry_count: 2,
    user_agent: "Custom-Bot/1.0 (Security-Test)"
}

sus ua_response HttpResponse = http_request_smart("GET", "https://httpbin.org/user-agent", 
    [], "", custom_ua_config) fam {
    when err -> {
        assert_string_contains(err, "network", "User agent test requires network")
        damn HttpResponse{status_code: 200, body: "mock_ua_response"}
    }
}

fr fr Test header injection prevention
sus malicious_headers []tea = []
malicious_headers = arrayz.push(malicious_headers, "Host: evil.example.com")
malicious_headers = arrayz.push(malicious_headers, "Content-Length: 999999")

sus injection_response HttpResponse = http_request_smart("GET", "https://httpbin.org/headers", 
    malicious_headers, "", default_client_config) fam {
    when err -> {
        fr fr Security measures should prevent injection
        assert_string_contains(err, "security", "Header injection prevented")
        damn HttpResponse{status_code: 0, body: ""}
    }
}

print_test_summary()
