// comprehensive_network_tests.csd - Comprehensive Network Module Test Suite
// Tests all enhanced networking functionality including sockets, pools, DNS, and HTTP

yeet "enhanced_networkz"
yeet "testz"
yeet "timez"
yeet "stringz"
yeet "arrayz"

// Test results tracking
sus test_results squad {
    sus total_tests drip = 0
    sus passed_tests drip = 0
    sus failed_tests drip = 0
    sus test_details []tea = []
}

slay add_test_result(test_name tea, passed lit, details tea) {
    test_results.total_tests = test_results.total_tests + 1
    
    ready (passed) {
        test_results.passed_tests = test_results.passed_tests + 1
    } otherwise {
        test_results.failed_tests = test_results.failed_tests + 1
    }
    
    sus status tea = ready passed { "PASS" } otherwise { "FAIL" }
    sus result_line tea = stringz.concat([status, " - ", test_name, ": ", details])
    test_results.test_details = arrayz.push(test_results.test_details, result_line)
}

// ==== SOCKET OPERATION TESTS ====

slay test_socket_creation() {
    vibez.spill("Testing socket creation...")
    
    // Test TCP socket creation
    sus tcp_socket Socket = socket_create(4, 1) fam {
        when err -> {
            add_test_result("TCP socket creation", no_cap, err.message)
            damn
        }
    }
    
    ready (tcp_socket.handle > 0 && tcp_socket.socket_type == 1 && tcp_socket.family == 4) {
        add_test_result("TCP socket creation", based, "TCP socket created successfully")
    } otherwise {
        add_test_result("TCP socket creation", no_cap, "Invalid TCP socket properties")
    }
    
    // Test UDP socket creation
    sus udp_socket Socket = socket_create(4, 2) fam {
        when err -> {
            add_test_result("UDP socket creation", no_cap, err.message)
            damn
        }
    }
    
    ready (udp_socket.handle > 0 && udp_socket.socket_type == 2) {
        add_test_result("UDP socket creation", based, "UDP socket created successfully")
    } otherwise {
        add_test_result("UDP socket creation", no_cap, "Invalid UDP socket properties")
    }
    
    // Test invalid socket parameters
    socket_create(10, 1) fam {
        when err -> {
            add_test_result("Invalid socket family", based, "Correctly rejected invalid family")
        }
        otherwise -> {
            add_test_result("Invalid socket family", no_cap, "Should have rejected invalid family")
        }
    }
    
    socket_create(4, 5) fam {
        when err -> {
            add_test_result("Invalid socket type", based, "Correctly rejected invalid type")
        }
        otherwise -> {
            add_test_result("Invalid socket type", no_cap, "Should have rejected invalid type")
        }
    }
}

slay test_socket_binding() {
    vibez.spill("Testing socket binding...")
    
    sus socket Socket = socket_create(4, 1) fam {
        when err -> {
            add_test_result("Socket bind setup", no_cap, err.message)
            damn
        }
    }
    
    // Test successful bind
    socket_bind(socket, "127.0.0.1", 8080) fam {
        when err -> {
            add_test_result("Socket bind success", no_cap, err.message)
        }
        otherwise -> {
            add_test_result("Socket bind success", based, "Socket bound to 127.0.0.1:8080")
        }
    }
    
    // Test invalid port
    socket_bind(socket, "127.0.0.1", 70000) fam {
        when err -> {
            add_test_result("Invalid port bind", based, "Correctly rejected invalid port")
        }
        otherwise -> {
            add_test_result("Invalid port bind", no_cap, "Should have rejected invalid port")
        }
    }
    
    // Test invalid IP
    socket_bind(socket, "300.300.300.300", 8081) fam {
        when err -> {
            add_test_result("Invalid IP bind", based, "Correctly rejected invalid IP")
        }
        otherwise -> {
            add_test_result("Invalid IP bind", no_cap, "Should have rejected invalid IP")
        }
    }
}

slay test_socket_connection() {
    vibez.spill("Testing socket connections...")
    
    sus socket Socket = socket_create(4, 1) fam {
        when err -> {
            add_test_result("Connection setup", no_cap, err.message)
            damn
        }
    }
    
    // Test successful connection to localhost
    socket_connect_with_timeout(socket, "localhost", 80, 30) fam {
        when err -> {
            add_test_result("Localhost connection", no_cap, err.message)
        }
        otherwise -> {
            add_test_result("Localhost connection", based, "Connected to localhost:80")
        }
    }
    
    // Test connection timeout
    sus timeout_socket Socket = socket_create(4, 1) fam {
        when err -> {
            add_test_result("Timeout test setup", no_cap, err.message)
            damn
        }
    }
    
    socket_connect_with_timeout(timeout_socket, "timeout.unreachable.example", 80, 1) fam {
        when err -> {
            add_test_result("Connection timeout", based, "Correctly timed out connection")
        }
        otherwise -> {
            add_test_result("Connection timeout", no_cap, "Should have timed out")
        }
    }
    
    // Test DNS resolution in connection
    sus dns_socket Socket = socket_create(4, 1) fam {
        when err -> {
            add_test_result("DNS connection setup", no_cap, err.message)
            damn
        }
    }
    
    socket_connect_with_timeout(dns_socket, "google.com", 80, 30) fam {
        when err -> {
            add_test_result("DNS resolution connection", no_cap, err.message)
        }
        otherwise -> {
            add_test_result("DNS resolution connection", based, "Connected via DNS resolution")
        }
    }
}

slay test_socket_data_transfer() {
    vibez.spill("Testing socket data transfer...")
    
    sus socket Socket = socket_create(4, 1) fam {
        when err -> {
            add_test_result("Data transfer setup", no_cap, err.message)
            damn
        }
    }
    
    socket_connect_with_timeout(socket, "127.0.0.1", 80, 30) fam {
        when err -> {
            add_test_result("Data transfer connection", no_cap, err.message)
            damn
        }
    }
    
    // Test sending data
    sus test_data tea = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n"
    sus bytes_sent drip = socket_send_data(socket, test_data) fam {
        when err -> {
            add_test_result("Socket send", no_cap, err.message)
            damn
        }
    }
    
    ready (bytes_sent == stringz.len(test_data)) {
        add_test_result("Socket send", based, stringz.concat(["Sent ", stringz.from_int(bytes_sent), " bytes"]))
    } otherwise {
        add_test_result("Socket send", no_cap, "Partial send or failure")
    }
    
    // Test receiving data
    sus received_data tea = socket_receive_data(socket, 1024) fam {
        when err -> {
            add_test_result("Socket receive", no_cap, err.message)
            damn
        }
    }
    
    ready (stringz.len(received_data) > 0) {
        add_test_result("Socket receive", based, stringz.concat(["Received ", stringz.from_int(stringz.len(received_data)), " bytes"]))
    } otherwise {
        add_test_result("Socket receive", no_cap, "No data received")
    }
    
    // Test empty send
    sus empty_bytes drip = socket_send_data(socket, "") fam {
        when err -> {
            add_test_result("Empty send", no_cap, err.message)
            damn
        }
    }
    
    ready (empty_bytes == 0) {
        add_test_result("Empty send", based, "Empty send returned 0 bytes")
    } otherwise {
        add_test_result("Empty send", no_cap, "Empty send should return 0")
    }
    
    socket_close(socket) fam { when _ -> {} }
}

// ==== CONNECTION POOL TESTS ====

slay test_connection_pool_creation() {
    vibez.spill("Testing connection pool creation...")
    
    // Test successful pool creation
    sus pool ConnectionPool = connection_pool_create("localhost", 80, "tcp", 10) fam {
        when err -> {
            add_test_result("Pool creation", no_cap, err.message)
            damn
        }
    }
    
    ready (stringz.equals(pool.host, "localhost") && pool.port == 80 && pool.max_connections == 10) {
        add_test_result("Pool creation", based, "Pool created with correct parameters")
    } otherwise {
        add_test_result("Pool creation", no_cap, "Pool parameters incorrect")
    }
    
    // Test invalid pool parameters
    connection_pool_create("localhost", 80, "tcp", 0) fam {
        when err -> {
            add_test_result("Invalid pool size", based, "Correctly rejected invalid pool size")
        }
        otherwise -> {
            add_test_result("Invalid pool size", no_cap, "Should reject invalid pool size")
        }
    }
    
    connection_pool_create("localhost", 80, "invalid", 10) fam {
        when err -> {
            add_test_result("Invalid protocol", based, "Correctly rejected invalid protocol")
        }
        otherwise -> {
            add_test_result("Invalid protocol", no_cap, "Should reject invalid protocol")
        }
    }
}

slay test_connection_pool_operations() {
    vibez.spill("Testing connection pool operations...")
    
    sus pool ConnectionPool = connection_pool_create("localhost", 80, "tcp", 3) fam {
        when err -> {
            add_test_result("Pool ops setup", no_cap, err.message)
            damn
        }
    }
    
    // Test getting connections from pool
    sus conn1 Socket = connection_pool_get_connection(pool) fam {
        when err -> {
            add_test_result("Pool get connection 1", no_cap, err.message)
            damn
        }
    }
    
    ready (conn1.handle > 0) {
        add_test_result("Pool get connection 1", based, "Got first connection from pool")
    } otherwise {
        add_test_result("Pool get connection 1", no_cap, "Invalid connection handle")
    }
    
    sus conn2 Socket = connection_pool_get_connection(pool) fam {
        when err -> {
            add_test_result("Pool get connection 2", no_cap, err.message)
            damn
        }
    }
    
    ready (conn2.handle > 0 && conn2.handle != conn1.handle) {
        add_test_result("Pool get connection 2", based, "Got second unique connection")
    } otherwise {
        add_test_result("Pool get connection 2", no_cap, "Connection not unique")
    }
    
    // Test returning connections
    connection_pool_return_connection(pool, conn1) fam {
        when err -> {
            add_test_result("Pool return connection", no_cap, err.message)
        }
        otherwise -> {
            add_test_result("Pool return connection", based, "Returned connection successfully")
        }
    }
    
    // Test pool health check
    connection_pool_health_check(pool) fam {
        when err -> {
            add_test_result("Pool health check", no_cap, err.message)
        }
        otherwise -> {
            add_test_result("Pool health check", based, "Health check completed")
        }
    }
}

// ==== DNS RESOLUTION TESTS ====

slay test_dns_resolution() {
    vibez.spill("Testing DNS resolution...")
    
    // Test resolving known hostnames
    sus localhost_ip tea = dns_resolve_hostname("localhost") fam {
        when err -> {
            add_test_result("DNS localhost", no_cap, err.message)
            damn
        }
    }
    
    ready (stringz.equals(localhost_ip, "127.0.0.1")) {
        add_test_result("DNS localhost", based, "Localhost resolved to 127.0.0.1")
    } otherwise {
        add_test_result("DNS localhost", no_cap, stringz.concat(["Localhost resolved to ", localhost_ip]))
    }
    
    sus google_ip tea = dns_resolve_hostname("google.com") fam {
        when err -> {
            add_test_result("DNS google.com", no_cap, err.message)
            damn
        }
    }
    
    ready (is_valid_ip_address(google_ip)) {
        add_test_result("DNS google.com", based, stringz.concat(["google.com resolved to ", google_ip]))
    } otherwise {
        add_test_result("DNS google.com", no_cap, "Invalid IP returned for google.com")
    }
    
    // Test IP address passthrough
    sus ip_passthrough tea = dns_resolve_hostname("8.8.8.8") fam {
        when err -> {
            add_test_result("DNS IP passthrough", no_cap, err.message)
            damn
        }
    }
    
    ready (stringz.equals(ip_passthrough, "8.8.8.8")) {
        add_test_result("DNS IP passthrough", based, "IP address passed through unchanged")
    } otherwise {
        add_test_result("DNS IP passthrough", no_cap, "IP passthrough failed")
    }
    
    // Test DNS cache
    sus start_time drip = timez.now()
    dns_resolve_hostname("github.com") fam { when _ -> {} }
    sus first_lookup_time drip = timez.now() - start_time
    
    start_time = timez.now()
    dns_resolve_hostname("github.com") fam { when _ -> {} }
    sus cached_lookup_time drip = timez.now() - start_time
    
    ready (cached_lookup_time <= first_lookup_time) {
        add_test_result("DNS caching", based, "Cached lookup was faster or equal")
    } otherwise {
        add_test_result("DNS caching", no_cap, "Cached lookup was slower")
    }
}

slay test_dns_record_types() {
    vibez.spill("Testing DNS record types...")
    
    // Test A records
    sus a_records []DNSRecord = dns_resolve_hostname_all_types("google.com", 1) fam {
        when err -> {
            add_test_result("DNS A records", no_cap, err.message)
            damn
        }
    }
    
    ready (arrayz.len(a_records) > 0 && a_records[0].record_type == 1) {
        add_test_result("DNS A records", based, stringz.concat(["Got ", stringz.from_int(arrayz.len(a_records)), " A records"]))
    } otherwise {
        add_test_result("DNS A records", no_cap, "No A records returned")
    }
    
    // Test MX records
    sus mx_records []DNSRecord = dns_resolve_hostname_all_types("gmail.com", 15) fam {
        when err -> {
            add_test_result("DNS MX records", no_cap, err.message)
            damn
        }
    }
    
    ready (arrayz.len(mx_records) > 0 && mx_records[0].record_type == 15) {
        add_test_result("DNS MX records", based, stringz.concat(["Got ", stringz.from_int(arrayz.len(mx_records)), " MX records"]))
    } otherwise {
        add_test_result("DNS MX records", no_cap, "No MX records returned")
    }
    
    // Test TXT records
    sus txt_records []DNSRecord = dns_resolve_hostname_all_types("google.com", 16) fam {
        when err -> {
            add_test_result("DNS TXT records", no_cap, err.message)
            damn
        }
    }
    
    ready (arrayz.len(txt_records) > 0 && txt_records[0].record_type == 16) {
        add_test_result("DNS TXT records", based, stringz.concat(["Got ", stringz.from_int(arrayz.len(txt_records)), " TXT records"]))
    } otherwise {
        add_test_result("DNS TXT records", no_cap, "No TXT records returned")
    }
    
    // Test unsupported record type
    dns_resolve_hostname_all_types("example.com", 99) fam {
        when err -> {
            add_test_result("DNS unsupported type", based, "Correctly rejected unsupported record type")
        }
        otherwise -> {
            add_test_result("DNS unsupported type", no_cap, "Should reject unsupported record type")
        }
    }
}

// ==== ADVANCED HTTP TESTS ====

slay test_advanced_http_get() {
    vibez.spill("Testing advanced HTTP GET...")
    
    // Test basic GET request
    sus headers []tea = ["Accept: application/json", "User-Agent: CURSED-Test/1.0"]
    sus response HttpResponseAdvanced = http_get_advanced("http://httpbin.org/get", headers, 30) fam {
        when err -> {
            add_test_result("HTTP GET basic", no_cap, err.message)
            damn
        }
    }
    
    ready (response.status_code == 200) {
        add_test_result("HTTP GET basic", based, stringz.concat(["HTTP GET returned status ", stringz.from_int(response.status_code)]))
    } otherwise {
        add_test_result("HTTP GET basic", no_cap, stringz.concat(["HTTP GET returned status ", stringz.from_int(response.status_code)]))
    }
    
    // Test GET with custom headers
    ready (stringz.len(response.content_type) > 0) {
        add_test_result("HTTP GET content-type", based, stringz.concat(["Content-Type: ", response.content_type]))
    } otherwise {
        add_test_result("HTTP GET content-type", no_cap, "No content-type header found")
    }
    
    ready (response.response_time_ms >= 0) {
        add_test_result("HTTP GET response time", based, stringz.concat(["Response time: ", stringz.from_int(response.response_time_ms), "ms"]))
    } otherwise {
        add_test_result("HTTP GET response time", no_cap, "Invalid response time")
    }
    
    // Test GET with timeout
    http_get_advanced("http://httpbin.org/delay/60", [], 1) fam {
        when err -> {
            add_test_result("HTTP GET timeout", based, "Correctly timed out request")
        }
        otherwise -> {
            add_test_result("HTTP GET timeout", no_cap, "Should have timed out")
        }
    }
}

slay test_advanced_http_post() {
    vibez.spill("Testing advanced HTTP POST...")
    
    sus json_body tea = "{\"name\":\"test\",\"value\":123,\"active\":true}"
    sus headers []tea = ["X-Test-Header: test-value"]
    
    sus response HttpResponseAdvanced = http_post_json_advanced("http://httpbin.org/post", json_body, headers, 30) fam {
        when err -> {
            add_test_result("HTTP POST JSON", no_cap, err.message)
            damn
        }
    }
    
    ready (response.status_code >= 200 && response.status_code < 300) {
        add_test_result("HTTP POST JSON", based, stringz.concat(["HTTP POST returned status ", stringz.from_int(response.status_code)]))
    } otherwise {
        add_test_result("HTTP POST JSON", no_cap, stringz.concat(["HTTP POST returned status ", stringz.from_int(response.status_code)]))
    }
    
    ready (stringz.contains(response.content_type, "json")) {
        add_test_result("HTTP POST content-type", based, "Response has JSON content-type")
    } otherwise {
        add_test_result("HTTP POST content-type", no_cap, stringz.concat(["Content-Type: ", response.content_type]))
    }
    
    ready (response.content_length > 0) {
        add_test_result("HTTP POST content-length", based, stringz.concat(["Content-Length: ", stringz.from_int(response.content_length)]))
    } otherwise {
        add_test_result("HTTP POST content-length", no_cap, "No content-length or zero")
    }
}

slay test_http_request_building() {
    vibez.spill("Testing HTTP request building...")
    
    sus request HttpRequestAdvanced = HttpRequestAdvanced{
        method: "POST",
        url: "https://api.example.com/data",
        headers: ["X-API-Key: secret123", "X-Version: 1.0"],
        body: "{\"test\":\"data\"}",
        timeout: 60,
        retry_count: 0,
        follow_redirects: based,
        user_agent: "CURSED-Test-Client/1.0",
        content_type: "application/json",
        authorization: "Bearer token123",
        cookies: ["session=abc123", "lang=en"],
        compression: "gzip"
    }
    
    sus raw_request tea = build_advanced_http_request(request) fam {
        when err -> {
            add_test_result("HTTP request building", no_cap, err.message)
            damn
        }
    }
    
    ready (stringz.contains(raw_request, "POST /data HTTP/1.1")) {
        add_test_result("HTTP request line", based, "Request line formatted correctly")
    } otherwise {
        add_test_result("HTTP request line", no_cap, "Request line incorrect")
    }
    
    ready (stringz.contains(raw_request, "Host: api.example.com")) {
        add_test_result("HTTP host header", based, "Host header present")
    } otherwise {
        add_test_result("HTTP host header", no_cap, "Host header missing")
    }
    
    ready (stringz.contains(raw_request, "User-Agent: CURSED-Test-Client/1.0")) {
        add_test_result("HTTP user agent", based, "User-Agent header correct")
    } otherwise {
        add_test_result("HTTP user agent", no_cap, "User-Agent header incorrect")
    }
    
    ready (stringz.contains(raw_request, "Authorization: Bearer token123")) {
        add_test_result("HTTP authorization", based, "Authorization header present")
    } otherwise {
        add_test_result("HTTP authorization", no_cap, "Authorization header missing")
    }
    
    ready (stringz.contains(raw_request, "Cookie: session=abc123; lang=en")) {
        add_test_result("HTTP cookies", based, "Cookie header formatted correctly")
    } otherwise {
        add_test_result("HTTP cookies", no_cap, "Cookie header incorrect")
    }
    
    ready (stringz.contains(raw_request, "Content-Length: 15")) {
        add_test_result("HTTP content length", based, "Content-Length calculated correctly")
    } otherwise {
        add_test_result("HTTP content length", no_cap, "Content-Length incorrect")
    }
}

slay test_http_response_parsing() {
    vibez.spill("Testing HTTP response parsing...")
    
    sus raw_response tea = "HTTP/1.1 201 Created\r\n" +
                           "Content-Type: application/json; charset=utf-8\r\n" +
                           "Content-Length: 45\r\n" +
                           "Set-Cookie: session=xyz789; Path=/\r\n" +
                           "Set-Cookie: lang=fr; Domain=.example.com\r\n" +
                           "Cache-Control: no-cache\r\n" +
                           "Content-Encoding: gzip\r\n" +
                           "\r\n" +
                           "{\"id\":123,\"status\":\"created\",\"success\":true}"
    
    sus response HttpResponseAdvanced = parse_advanced_http_response(raw_response) fam {
        when err -> {
            add_test_result("HTTP response parsing", no_cap, err.message)
            damn
        }
    }
    
    ready (response.status_code == 201) {
        add_test_result("HTTP status code", based, "Status code parsed correctly (201)")
    } otherwise {
        add_test_result("HTTP status code", no_cap, stringz.concat(["Status code: ", stringz.from_int(response.status_code)]))
    }
    
    ready (stringz.equals(response.status_message, "Created")) {
        add_test_result("HTTP status message", based, "Status message parsed correctly")
    } otherwise {
        add_test_result("HTTP status message", no_cap, stringz.concat(["Status message: ", response.status_message]))
    }
    
    ready (stringz.contains(response.content_type, "application/json")) {
        add_test_result("HTTP response content-type", based, "Content-Type extracted correctly")
    } otherwise {
        add_test_result("HTTP response content-type", no_cap, stringz.concat(["Content-Type: ", response.content_type]))
    }
    
    ready (response.content_length == 45) {
        add_test_result("HTTP response content-length", based, "Content-Length parsed correctly")
    } otherwise {
        add_test_result("HTTP response content-length", no_cap, stringz.concat(["Content-Length: ", stringz.from_int(response.content_length)]))
    }
    
    ready (arrayz.len(response.cookies) == 2) {
        add_test_result("HTTP response cookies", based, "Cookies parsed correctly")
    } otherwise {
        add_test_result("HTTP response cookies", no_cap, stringz.concat(["Cookies count: ", stringz.from_int(arrayz.len(response.cookies))]))
    }
    
    ready (stringz.equals(response.cache_control, "no-cache")) {
        add_test_result("HTTP cache control", based, "Cache-Control parsed correctly")
    } otherwise {
        add_test_result("HTTP cache control", no_cap, stringz.concat(["Cache-Control: ", response.cache_control]))
    }
    
    ready (stringz.equals(response.compression, "gzip")) {
        add_test_result("HTTP compression", based, "Content-Encoding parsed correctly")
    } otherwise {
        add_test_result("HTTP compression", no_cap, stringz.concat(["Compression: ", response.compression]))
    }
    
    ready (stringz.contains(response.body, "\"success\":true")) {
        add_test_result("HTTP response body", based, "Response body parsed correctly")
    } otherwise {
        add_test_result("HTTP response body", no_cap, "Response body incorrect")
    }
}

// ==== UTILITY FUNCTION TESTS ====

slay test_utility_functions() {
    vibez.spill("Testing utility functions...")
    
    // Test IP address validation
    ready (is_valid_ip_address("192.168.1.1")) {
        add_test_result("IP validation - valid IPv4", based, "192.168.1.1 is valid")
    } otherwise {
        add_test_result("IP validation - valid IPv4", no_cap, "192.168.1.1 should be valid")
    }
    
    ready (!is_valid_ip_address("300.300.300.300")) {
        add_test_result("IP validation - invalid IPv4", based, "300.300.300.300 is invalid")
    } otherwise {
        add_test_result("IP validation - invalid IPv4", no_cap, "300.300.300.300 should be invalid")
    }
    
    ready (is_valid_ip_address("::1")) {
        add_test_result("IP validation - valid IPv6", based, "::1 is valid")
    } otherwise {
        add_test_result("IP validation - valid IPv6", no_cap, "::1 should be valid")
    }
    
    ready (!is_valid_ip_address("not.an.ip.address")) {
        add_test_result("IP validation - invalid format", based, "Invalid format rejected")
    } otherwise {
        add_test_result("IP validation - invalid format", no_cap, "Invalid format should be rejected")
    }
    
    // Test port usage check
    ready (is_port_in_use("127.0.0.1", 22)) {
        add_test_result("Port usage - system port", based, "System port detected as in use")
    } otherwise {
        add_test_result("Port usage - system port", no_cap, "System port should be in use")
    }
    
    ready (!is_port_in_use("127.0.0.1", 54321)) {
        add_test_result("Port usage - high port", based, "High port detected as available")
    } otherwise {
        add_test_result("Port usage - high port", no_cap, "High port should be available")
    }
}

// ==== ERROR HANDLING TESTS ====

slay test_error_handling() {
    vibez.spill("Testing error handling...")
    
    // Test retryable errors
    sus timeout_error NetworkError = create_network_error_advanced("timeout", "Connection timeout", 408, "")
    ready (is_retryable_error(timeout_error)) {
        add_test_result("Retryable error - timeout", based, "Timeout error is retryable")
    } otherwise {
        add_test_result("Retryable error - timeout", no_cap, "Timeout error should be retryable")
    }
    
    sus not_found_error NetworkError = create_network_error_advanced("not_found", "Not found", 404, "")
    ready (!is_retryable_error(not_found_error)) {
        add_test_result("Non-retryable error", based, "404 error is not retryable")
    } otherwise {
        add_test_result("Non-retryable error", no_cap, "404 error should not be retryable")
    }
    
    // Test retry logic
    timeout_error.retry_count = 2
    ready (should_retry(timeout_error, 3)) {
        add_test_result("Should retry - under limit", based, "Should retry when under limit")
    } otherwise {
        add_test_result("Should retry - under limit", no_cap, "Should retry when under limit")
    }
    
    timeout_error.retry_count = 3
    ready (!should_retry(timeout_error, 3)) {
        add_test_result("Should not retry - at limit", based, "Should not retry at limit")
    } otherwise {
        add_test_result("Should not retry - at limit", no_cap, "Should not retry at limit")
    }
}

// ==== NETWORK STATISTICS TESTS ====

slay test_network_statistics() {
    vibez.spill("Testing network statistics...")
    
    sus initial_stats NetworkStats = get_network_statistics()
    
    // Create some network activity
    sus socket Socket = socket_create(4, 1) fam { when _ -> {} }
    socket_connect_with_timeout(socket, "localhost", 80, 30) fam { when _ -> {} }
    socket_send_data(socket, "test data") fam { when _ -> {} }
    socket_close(socket) fam { when _ -> {} }
    
    sus pool ConnectionPool = connection_pool_create("localhost", 80, "tcp", 5) fam { when _ -> {} }
    connection_pool_get_connection(pool) fam { when _ -> {} }
    
    dns_resolve_hostname("test.example.com") fam { when _ -> {} }
    
    sus final_stats NetworkStats = get_network_statistics()
    
    ready (final_stats.total_tcp_connections > initial_stats.total_tcp_connections) {
        add_test_result("Network stats - TCP connections", based, "TCP connection count increased")
    } otherwise {
        add_test_result("Network stats - TCP connections", no_cap, "TCP connection count should have increased")
    }
    
    ready (final_stats.dns_queries > initial_stats.dns_queries) {
        add_test_result("Network stats - DNS queries", based, "DNS query count increased")
    } otherwise {
        add_test_result("Network stats - DNS queries", no_cap, "DNS query count should have increased")
    }
    
    // Test statistics reset
    reset_network_statistics() fam { when _ -> {} }
    sus reset_stats NetworkStats = get_network_statistics()
    
    ready (reset_stats.total_tcp_connections == 0) {
        add_test_result("Network stats - reset", based, "Statistics reset successfully")
    } otherwise {
        add_test_result("Network stats - reset", no_cap, "Statistics should reset to zero")
    }
}

// ==== COMPREHENSIVE INTEGRATION TESTS ====

slay test_end_to_end_http_workflow() {
    vibez.spill("Testing end-to-end HTTP workflow...")
    
    // Full workflow: DNS resolution -> connection pool -> HTTP request -> response processing
    sus pool ConnectionPool = connection_pool_create("httpbin.org", 80, "tcp", 3) fam {
        when err -> {
            add_test_result("E2E setup", no_cap, err.message)
            damn
        }
    }
    
    sus request HttpRequestAdvanced = HttpRequestAdvanced{
        method: "GET",
        url: "http://httpbin.org/json",
        headers: ["Accept: application/json"],
        body: "",
        timeout: 30,
        retry_count: 0,
        follow_redirects: based,
        user_agent: "CURSED-E2E-Test/1.0",
        content_type: "",
        authorization: "",
        cookies: [],
        compression: "gzip"
    }
    
    sus response HttpResponseAdvanced = http_request_with_pool(request, pool) fam {
        when err -> {
            add_test_result("E2E HTTP request", no_cap, err.message)
            damn
        }
    }
    
    ready (response.status_code == 200) {
        add_test_result("E2E status code", based, "E2E request returned 200 OK")
    } otherwise {
        add_test_result("E2E status code", no_cap, stringz.concat(["E2E request returned ", stringz.from_int(response.status_code)]))
    }
    
    ready (stringz.contains(response.content_type, "json")) {
        add_test_result("E2E content type", based, "E2E response has JSON content type")
    } otherwise {
        add_test_result("E2E content type", no_cap, "E2E response should be JSON")
    }
    
    ready (stringz.len(response.body) > 0) {
        add_test_result("E2E response body", based, "E2E response has content")
    } otherwise {
        add_test_result("E2E response body", no_cap, "E2E response should have content")
    }
    
    ready (response.response_time_ms >= 0) {
        add_test_result("E2E response time", based, stringz.concat(["E2E response time: ", stringz.from_int(response.response_time_ms), "ms"]))
    } otherwise {
        add_test_result("E2E response time", no_cap, "E2E response time should be valid")
    }
}

// ==== TEST RUNNER ====

slay run_all_network_tests() {
    vibez.spill("🚀 Starting Comprehensive Network Module Test Suite")
    vibez.spill("=" * 60)
    
    sus start_time drip = timez.now()
    
    // Initialize test results
    test_results.total_tests = 0
    test_results.passed_tests = 0
    test_results.failed_tests = 0
    test_results.test_details = []
    
    // Run all test categories
    test_socket_creation()
    test_socket_binding()
    test_socket_connection()
    test_socket_data_transfer()
    
    test_connection_pool_creation()
    test_connection_pool_operations()
    
    test_dns_resolution()
    test_dns_record_types()
    
    test_advanced_http_get()
    test_advanced_http_post()
    test_http_request_building()
    test_http_response_parsing()
    
    test_utility_functions()
    test_error_handling()
    test_network_statistics()
    
    test_end_to_end_http_workflow()
    
    sus end_time drip = timez.now()
    sus total_time drip = end_time - start_time
    
    // Print results summary
    vibez.spill("")
    vibez.spill("=" * 60)
    vibez.spill("🏁 TEST RESULTS SUMMARY")
    vibez.spill("=" * 60)
    vibez.spill(stringz.concat(["Total Tests: ", stringz.from_int(test_results.total_tests)]))
    vibez.spill(stringz.concat(["Passed: ", stringz.from_int(test_results.passed_tests)]))
    vibez.spill(stringz.concat(["Failed: ", stringz.from_int(test_results.failed_tests)]))
    
    sus pass_rate drip = ready test_results.total_tests > 0 { 
        (test_results.passed_tests * 100) / test_results.total_tests 
    } otherwise { 0 }
    vibez.spill(stringz.concat(["Pass Rate: ", stringz.from_int(pass_rate), "%"]))
    vibez.spill(stringz.concat(["Total Time: ", stringz.from_int(total_time), "ms"]))
    
    vibez.spill("")
    vibez.spill("📋 DETAILED RESULTS:")
    vibez.spill("-" * 60)
    
    sus i drip = 0
    bestie (i < arrayz.len(test_results.test_details)) {
        vibez.spill(test_results.test_details[i])
        i = i + 1
    }
    
    vibez.spill("")
    vibez.spill("=" * 60)
    
    ready (test_results.failed_tests == 0) {
        vibez.spill("✅ ALL TESTS PASSED! Network module is production ready.")
    } otherwise {
        vibez.spill("❌ SOME TESTS FAILED. Review failed tests above.")
    }
    
    vibez.spill("=" * 60)
    
    // Network statistics summary
    sus final_stats NetworkStats = get_network_statistics()
    vibez.spill("")
    vibez.spill("📊 NETWORK STATISTICS SUMMARY:")
    vibez.spill(stringz.concat(["TCP Connections: ", stringz.from_int(final_stats.total_tcp_connections)]))
    vibez.spill(stringz.concat(["UDP Connections: ", stringz.from_int(final_stats.total_udp_connections)]))
    vibez.spill(stringz.concat(["Bytes Sent: ", stringz.from_int(final_stats.total_bytes_sent)]))
    vibez.spill(stringz.concat(["Bytes Received: ", stringz.from_int(final_stats.total_bytes_received)]))
    vibez.spill(stringz.concat(["DNS Queries: ", stringz.from_int(final_stats.dns_queries)]))
    vibez.spill(stringz.concat(["DNS Cache Hits: ", stringz.from_int(final_stats.dns_cache_hits)]))
    vibez.spill(stringz.concat(["Failed Connections: ", stringz.from_int(final_stats.failed_connections)]))
    vibez.spill(stringz.concat(["Successful Connections: ", stringz.from_int(final_stats.successful_connections)]))
}

// Run the tests
run_all_network_tests()
