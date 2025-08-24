fr fr CURSED URL Parsing Package (urlz) - Comprehensive Test Suite
fr fr Tests all URL parsing, validation, encoding, and manipulation functions

yeet "urlz"
yeet "testz"

fr fr ===== URL PARSING TESTS =====

slay test_basic_url_parsing() {
    testz.test_group("Basic URL Parsing")
    
    fr fr Test simple HTTP URL
    sus url urlz.URL = urlz.parse_url("http://example.com")
    testz.assert_eq_str(url.scheme, "http")
    testz.assert_eq_str(url.host, "example.com")
    testz.assert_eq_int(url.port, 80)
    testz.assert_eq_str(url.path, "/")
    testz.assert_lit(url.is_valid, based)
    
    fr fr Test HTTPS URL with path
    sus https_url urlz.URL = urlz.parse_url("https://api.example.com/v1/users")
    testz.assert_eq_str(https_url.scheme, "https")
    testz.assert_eq_str(https_url.host, "api.example.com")
    testz.assert_eq_int(https_url.port, 443)
    testz.assert_eq_str(https_url.path, "/v1/users")
    testz.assert_lit(https_url.is_valid, based)
}

slay test_url_with_port() {
    testz.test_group("URL with Custom Port")
    
    sus url urlz.URL = urlz.parse_url("http://localhost:8080/app")
    testz.assert_eq_str(url.scheme, "http")
    testz.assert_eq_str(url.host, "localhost")
    testz.assert_eq_int(url.port, 8080)
    testz.assert_eq_str(url.path, "/app")
    testz.assert_lit(url.is_valid, based)
}

slay test_url_with_query_and_fragment() {
    testz.test_group("URL with Query and Fragment")
    
    sus url urlz.URL = urlz.parse_url("https://example.com/search?q=test&page=1#results")
    testz.assert_eq_str(url.scheme, "https")
    testz.assert_eq_str(url.host, "example.com")
    testz.assert_eq_str(url.path, "/search")
    testz.assert_eq_str(url.query, "q=test&page=1")
    testz.assert_eq_str(url.fragment, "results")
    testz.assert_lit(url.is_valid, based)
}

slay test_url_with_authentication() {
    testz.test_group("URL with Authentication")
    
    sus url urlz.URL = urlz.parse_url("https://user:pass@api.example.com/secure")
    testz.assert_eq_str(url.scheme, "https")
    testz.assert_eq_str(url.username, "user")
    testz.assert_eq_str(url.password, "pass")
    testz.assert_eq_str(url.host, "api.example.com")
    testz.assert_eq_str(url.path, "/secure")
    testz.assert_lit(url.is_valid, based)
}

slay test_complex_url() {
    testz.test_group("Complex URL Parsing")
    
    sus complex tea = "https://admin:secret@api.myapp.com:8443/v2/users/123?include=profile&format=json#user-details"
    sus url urlz.URL = urlz.parse_url(complex)
    
    testz.assert_eq_str(url.scheme, "https")
    testz.assert_eq_str(url.username, "admin")
    testz.assert_eq_str(url.password, "secret")
    testz.assert_eq_str(url.host, "api.myapp.com")
    testz.assert_eq_int(url.port, 8443)
    testz.assert_eq_str(url.path, "/v2/users/123")
    testz.assert_eq_str(url.query, "include=profile&format=json")
    testz.assert_eq_str(url.fragment, "user-details")
    testz.assert_lit(url.is_valid, based)
}

fr fr ===== URL VALIDATION TESTS =====

slay test_url_validation() {
    testz.test_group("URL Validation")
    
    fr fr Valid URLs
    testz.assert_lit(urlz.is_valid_url("http://example.com"), based)
    testz.assert_lit(urlz.is_valid_url("https://api.example.com/v1"), based)
    testz.assert_lit(urlz.is_valid_url("ftp://files.example.com"), based)
    
    fr fr Invalid URLs
    testz.assert_lit(urlz.is_valid_url(""), cringe)
    testz.assert_lit(urlz.is_valid_url("not-a-url"), cringe)
    testz.assert_lit(urlz.is_valid_url("http://"), cringe)
}

slay test_url_type_checks() {
    testz.test_group("URL Type Checks")
    
    fr fr Absolute URLs
    testz.assert_lit(urlz.is_absolute_url("http://example.com"), based)
    testz.assert_lit(urlz.is_absolute_url("https://example.com/path"), based)
    testz.assert_lit(urlz.is_absolute_url("/relative/path"), cringe)
    testz.assert_lit(urlz.is_absolute_url("relative/path"), cringe)
    
    fr fr Relative URLs
    testz.assert_lit(urlz.is_relative_url("/relative/path"), based)
    testz.assert_lit(urlz.is_relative_url("relative/path"), based)
    testz.assert_lit(urlz.is_relative_url("http://example.com"), cringe)
    
    fr fr Secure URLs
    testz.assert_lit(urlz.is_secure_url("https://example.com"), based)
    testz.assert_lit(urlz.is_secure_url("http://example.com"), cringe)
    testz.assert_lit(urlz.is_secure_url("ftp://example.com"), cringe)
}

fr fr ===== URL BUILDING TESTS =====

slay test_url_building() {
    testz.test_group("URL Building")
    
    sus url urlz.URL = urlz.URL{
        scheme: "https",
        host: "example.com",
        port: 443,
        path: "/api/v1",
        query: "format=json",
        fragment: "section1",
        username: "",
        password: "",
        is_valid: based
    }
    
    sus built tea = urlz.build_url(url)
    sus expected tea = "https://example.com/api/v1?format=json#section1"
    testz.assert_eq_str(built, expected)
}

slay test_url_building_with_custom_port() {
    testz.test_group("URL Building with Custom Port")
    
    sus url urlz.URL = urlz.URL{
        scheme: "http",
        host: "localhost",
        port: 8080,
        path: "/dev",
        query: "",
        fragment: "",
        username: "",
        password: "",
        is_valid: based
    }
    
    sus built tea = urlz.build_url(url)
    sus expected tea = "http://localhost:8080/dev"
    testz.assert_eq_str(built, expected)
}

fr fr ===== URL ENCODING TESTS =====

slay test_url_encoding() {
    testz.test_group("URL Encoding")
    
    fr fr Basic encoding
    testz.assert_eq_str(urlz.url_encode("hello world"), "hello%20world")
    testz.assert_eq_str(urlz.url_encode("test@example.com"), "test%40example.com")
    testz.assert_eq_str(urlz.url_encode("a+b=c"), "a%2Bb%3Dc")
    
    fr fr Characters that don't need encoding
    testz.assert_eq_str(urlz.url_encode("abc123"), "abc123")
    testz.assert_eq_str(urlz.url_encode("test-file_name.txt"), "test-file_name.txt")
}

slay test_url_decoding() {
    testz.test_group("URL Decoding")
    
    fr fr Basic decoding
    testz.assert_eq_str(urlz.url_decode("hello%20world"), "hello world")
    testz.assert_eq_str(urlz.url_decode("test%40example.com"), "test@example.com")
    testz.assert_eq_str(urlz.url_decode("a%2Bb%3Dc"), "a+b=c")
    
    fr fr Plus sign handling
    testz.assert_eq_str(urlz.url_decode("hello+world"), "hello world")
    
    fr fr Characters that don't need decoding
    testz.assert_eq_str(urlz.url_decode("abc123"), "abc123")
}

slay test_encoding_decoding_roundtrip() {
    testz.test_group("Encoding/Decoding Roundtrip")
    
    sus original tea = "hello world! @#$%^&*()"
    sus encoded tea = urlz.url_encode(original)
    sus decoded tea = urlz.url_decode(encoded)
    testz.assert_eq_str(decoded, original)
}

fr fr ===== QUERY STRING TESTS =====

slay test_query_string_parsing() {
    testz.test_group("Query String Parsing")
    
    sus query tea = "name=John&age=30&city=NYC"
    sus params []urlz.QueryParam = urlz.parse_query_string(query)
    
    testz.assert_eq_str(params[0].key, "name")
    testz.assert_eq_str(params[0].value, "John")
}

slay test_query_string_building() {
    testz.test_group("Query String Building")
    
    sus params []urlz.QueryParam = [
        urlz.QueryParam{key: "search", value: "cursed lang"},
        urlz.QueryParam{key: "page", value: "1"}
    ]
    
    sus query tea = urlz.build_query_string(params)
    sus expected tea = "search=cursed%20lang&page=1"
    testz.assert_eq_str(query, expected)
}

slay test_query_param_operations() {
    testz.test_group("Query Parameter Operations")
    
    sus query tea = "name=John&age=30"
    
    fr fr Get parameter
    testz.assert_eq_str(urlz.get_query_param(query, "name"), "John")
    testz.assert_eq_str(urlz.get_query_param(query, "age"), "30")
    testz.assert_eq_str(urlz.get_query_param(query, "missing"), "")
    
    fr fr Check parameter existence
    testz.assert_lit(urlz.has_query_param(query, "name"), based)
    testz.assert_lit(urlz.has_query_param(query, "missing"), cringe)
    
    fr fr Set parameter
    sus updated tea = urlz.set_query_param(query, "age", "31")
    testz.assert_eq_str(urlz.get_query_param(updated, "age"), "31")
    
    fr fr Remove parameter
    sus removed tea = urlz.remove_query_param(query, "age")
    testz.assert_lit(urlz.has_query_param(removed, "age"), cringe)
    testz.assert_lit(urlz.has_query_param(removed, "name"), based)
}

fr fr ===== URL MANIPULATION TESTS =====

slay test_path_joining() {
    testz.test_group("Path Joining")
    
    testz.assert_eq_str(urlz.join_url_paths("/api/v1", "users"), "/api/v1/users")
    testz.assert_eq_str(urlz.join_url_paths("/api/v1/", "/users"), "/api/v1/users")
    testz.assert_eq_str(urlz.join_url_paths("", "users"), "users")
    testz.assert_eq_str(urlz.join_url_paths("/api", ""), "/api")
}

slay test_url_normalization() {
    testz.test_group("URL Normalization")
    
    fr fr Case normalization
    sus normalized tea = urlz.normalize_url("HTTPS://Example.COM/Path")
    testz.assert_eq_str(normalized, "https://example.com/Path")
    
    fr fr Path normalization
    sus path_normalized tea = urlz.normalize_url("https://example.com//path//to//file")
    testz.assert_eq_str(path_normalized, "https://example.com/path/to/file")
}

slay test_relative_url_resolution() {
    testz.test_group("Relative URL Resolution")
    
    sus base tea = "https://example.com/api/v1/users"
    
    fr fr Absolute path
    sus resolved1 tea = urlz.resolve_relative_url(base, "/docs/guide")
    testz.assert_eq_str(resolved1, "https://example.com/docs/guide")
    
    fr fr Relative path
    sus resolved2 tea = urlz.resolve_relative_url(base, "123/profile")
    testz.assert_eq_str(resolved2, "https://example.com/api/v1/users/123/profile")
    
    fr fr Query only
    sus resolved3 tea = urlz.resolve_relative_url(base, "?format=json")
    testz.assert_eq_str(resolved3, "https://example.com/api/v1/users?format=json")
    
    fr fr Fragment only
    sus resolved4 tea = urlz.resolve_relative_url(base, "#section1")
    testz.assert_eq_str(resolved4, "https://example.com/api/v1/users#section1")
}

fr fr ===== URL COMPONENT EXTRACTION TESTS =====

slay test_url_component_extraction() {
    testz.test_group("URL Component Extraction")
    
    sus url tea = "https://api.subdomain.example.com:8080/path?query=value"
    
    fr fr Domain extraction
    testz.assert_eq_str(urlz.get_domain(url), "api.subdomain.example.com")
    testz.assert_eq_str(urlz.get_subdomain(url), "api")
    testz.assert_eq_str(urlz.get_top_level_domain(url), "com")
    
    fr fr Base URL
    testz.assert_eq_str(urlz.get_base_url(url), "https://api.subdomain.example.com:8080")
    testz.assert_eq_str(urlz.get_origin(url), "https://api.subdomain.example.com:8080")
}

fr fr ===== URL COMPARISON TESTS =====

slay test_url_comparison() {
    testz.test_group("URL Comparison")
    
    fr fr URL equality (with normalization)
    testz.assert_lit(urlz.urls_equal("https://example.com/", "https://EXAMPLE.COM"), based)
    testz.assert_lit(urlz.urls_equal("http://example.com/path", "http://example.com/different"), cringe)
    
    fr fr Same origin
    testz.assert_lit(urlz.same_origin("https://example.com/api", "https://example.com/docs"), based)
    testz.assert_lit(urlz.same_origin("https://example.com", "http://example.com"), cringe)
    
    fr fr Same domain
    testz.assert_lit(urlz.is_same_domain("https://example.com/api", "http://example.com/docs"), based)
    testz.assert_lit(urlz.is_same_domain("https://example.com", "https://other.com"), cringe)
}

slay test_url_pattern_matching() {
    testz.test_group("URL Pattern Matching")
    
    fr fr Wildcard patterns
    testz.assert_lit(urlz.matches_pattern("https://example.com/api", "*"), based)
    testz.assert_lit(urlz.matches_pattern("https://example.com/api/users", "*.com/api/*"), based)
    testz.assert_lit(urlz.matches_pattern("https://example.com", "https://example.com"), based)
    testz.assert_lit(urlz.matches_pattern("https://example.com/api", "https://other.com/*"), cringe)
}

fr fr ===== SECURITY TESTS =====

slay test_redirect_safety() {
    testz.test_group("Redirect Safety")
    
    sus allowed_hosts []tea = ["example.com", "api.example.com", "secure.myapp.com"]
    
    fr fr Safe redirects
    testz.assert_lit(urlz.is_safe_redirect("https://example.com/login", allowed_hosts), based)
    testz.assert_lit(urlz.is_safe_redirect("https://api.example.com/callback", allowed_hosts), based)
    
    fr fr Unsafe redirects
    testz.assert_lit(urlz.is_safe_redirect("https://malicious.com/phish", allowed_hosts), cringe)
    testz.assert_lit(urlz.is_safe_redirect("javascript:alert('xss')", allowed_hosts), cringe)
    testz.assert_lit(urlz.is_safe_redirect("", allowed_hosts), cringe)
}

slay test_url_sanitization() {
    testz.test_group("URL Sanitization")
    
    fr fr Safe URLs pass through
    testz.assert_eq_str(urlz.sanitize_url("https://example.com/safe"), "https://example.com/safe")
    testz.assert_eq_str(urlz.sanitize_url("http://example.com/api"), "http://example.com/api")
    
    fr fr Dangerous schemes blocked
    testz.assert_eq_str(urlz.sanitize_url("javascript:alert('xss')"), "")
    testz.assert_eq_str(urlz.sanitize_url("data:text/html,<script>"), "")
    
    fr fr Credentials removed
    sus sanitized tea = urlz.sanitize_url("https://user:pass@example.com/api")
    testz.assert_eq_str(sanitized, "https://example.com/api")
}

fr fr ===== EDGE CASES AND ERROR HANDLING =====

slay test_edge_cases() {
    testz.test_group("Edge Cases")
    
    fr fr Empty URL
    sus empty_url urlz.URL = urlz.parse_url("")
    testz.assert_lit(empty_url.is_valid, cringe)
    
    fr fr Invalid URL
    sus invalid_url urlz.URL = urlz.parse_url("not-a-url")
    testz.assert_lit(invalid_url.is_valid, cringe)
    
    fr fr Empty query string
    sus empty_params []urlz.QueryParam = urlz.parse_query_string("")
    testz.assert_eq_int(urlz.len(empty_params), 0)
    
    fr fr Missing query parameter
    testz.assert_eq_str(urlz.get_query_param("name=John", "missing"), "")
}

fr fr ===== PERFORMANCE TESTS =====

slay test_performance() {
    testz.test_group("Performance Tests")
    
    fr fr Parse many URLs
    sus i drip = 0
    bestie (i < 100) {
        sus url urlz.URL = urlz.parse_url("https://api.example.com/v1/users/" + urlz.int_to_string(i))
        testz.assert_lit(url.is_valid, based)
        i = i + 1
    }
    
    fr fr Encode/decode many strings
    sus j drip = 0
    bestie (j < 50) {
        sus original tea = "test string " + urlz.int_to_string(j) + " with spaces!"
        sus encoded tea = urlz.url_encode(original)
        sus decoded tea = urlz.url_decode(encoded)
        testz.assert_eq_str(decoded, original)
        j = j + 1
    }
}

fr fr ===== INTEGRATION TESTS =====

slay test_real_world_scenarios() {
    testz.test_group("Real World Scenarios")
    
    fr fr API endpoint construction
    sus base_api tea = "https://api.myapp.com/v1"
    sus user_id tea = "123"
    sus user_endpoint tea = urlz.join_url_paths(base_api, "users/" + user_id)
    
    sus query_params []urlz.QueryParam = [
        urlz.QueryParam{key: "include", value: "profile,settings"},
        urlz.QueryParam{key: "format", value: "json"}
    ]
    
    sus query tea = urlz.build_query_string(query_params)
    sus final_url tea = user_endpoint + "?" + query
    
    testz.assert_eq_str(final_url, "https://api.myapp.com/v1/users/123?include=profile%2Csettings&format=json")
    
    fr fr URL validation for redirect
    sus redirect_url tea = "https://trusted.myapp.com/dashboard"
    sus allowed []tea = ["myapp.com", "trusted.myapp.com", "api.myapp.com"]
    testz.assert_lit(urlz.is_safe_redirect(redirect_url, allowed), based)
}

fr fr ===== MAIN TEST RUNNER =====

slay main() drip {
    testz.test_start("URL Parsing Package (urlz) Test Suite")
    
    fr fr URL Parsing Tests
    test_basic_url_parsing()
    test_url_with_port()
    test_url_with_query_and_fragment()
    test_url_with_authentication()
    test_complex_url()
    
    fr fr URL Validation Tests
    test_url_validation()
    test_url_type_checks()
    
    fr fr URL Building Tests
    test_url_building()
    test_url_building_with_custom_port()
    
    fr fr URL Encoding Tests
    test_url_encoding()
    test_url_decoding()
    test_encoding_decoding_roundtrip()
    
    fr fr Query String Tests
    test_query_string_parsing()
    test_query_string_building()
    test_query_param_operations()
    
    fr fr URL Manipulation Tests
    test_path_joining()
    test_url_normalization()
    test_relative_url_resolution()
    
    fr fr Component Extraction Tests
    test_url_component_extraction()
    
    fr fr URL Comparison Tests
    test_url_comparison()
    test_url_pattern_matching()
    
    fr fr Security Tests
    test_redirect_safety()
    test_url_sanitization()
    
    fr fr Edge Cases and Performance
    test_edge_cases()
    test_performance()
    
    fr fr Integration Tests
    test_real_world_scenarios()
    
    testz.print_test_summary()
    damn 0
}
