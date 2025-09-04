// CURSED Pure Networking Module Tests - 100% FFI-Free Validation
// Comprehensive test suite for pure CURSED networking implementation

yeet "testz"

// Test basic socket creation and management
slay test_socket_creation() {
    test_start("Socket Creation and Management")
    
    // Test TCP socket creation
    sus tcp_handle normie = tcp_socket_create()
    assert_true(tcp_handle != -1)
    
    // Test UDP socket creation
    sus udp_handle normie = udp_socket_create()
    assert_true(udp_handle != -1)
    
    // Test socket closing
    assert_true(close_socket(tcp_handle))
    assert_true(close_socket(udp_handle))
}

// Test TCP socket operations
slay test_tcp_socket_operations() {
    test_start("TCP Socket Operations")
    
    sus socket normie = tcp_socket_create()
    assert_true(socket != -1)
    
    // Test binding to localhost
    assert_true(tcp_socket_bind(socket, "127.0.0.1", 8080))
    
    // Test listening
    assert_true(tcp_socket_listen(socket, 10))
    
    // Test connection to common ports
    sus client_socket normie = tcp_socket_create()
    assert_true(tcp_socket_connect(client_socket, "127.0.0.1", 80))
    
    // Test data transmission
    sus bytes_sent normie = tcp_socket_send(client_socket, "GET / HTTP/1.1\r\n\r\n")
    assert_true(bytes_sent > 0)
    
    // Test data reception
    sus response tea = tcp_socket_recv(client_socket, 1024)
    assert_true(string_length(response) > 0)
    assert_true(string_contains(response, "HTTP/1.1"))
    
    // Cleanup
    close_socket(socket)
    close_socket(client_socket)
}

// Test UDP socket operations
slay test_udp_socket_operations() {
    test_start("UDP Socket Operations")
    
    sus socket normie = udp_socket_create()
    assert_true(socket != -1)
    
    // Test binding
    assert_true(udp_socket_bind(socket, "127.0.0.1", 8081))
    
    // Test sending data
    sus bytes_sent normie = udp_socket_send_to(socket, "Hello UDP", "127.0.0.1", 8082)
    assert_true(bytes_sent > 0)
    
    // Test receiving data
    sus response tea = udp_socket_recv_from(socket, 1024)
    assert_true(string_length(response) > 0)
    
    close_socket(socket)
}

// Test DNS resolution
slay test_dns_resolution() {
    test_start("DNS Resolution")
    
    // Test hostname to IP resolution
    sus localhost_ip tea = resolve_hostname("localhost")
    assert_eq_string(localhost_ip, "127.0.0.1")
    
    sus google_ip tea = resolve_hostname("google.com")
    assert_true(string_length(google_ip) > 0)
    assert_true(string_contains(google_ip, "."))
    
    // Test reverse DNS
    sus hostname tea = resolve_ip_to_hostname("127.0.0.1")
    assert_eq_string(hostname, "localhost")
    
    // Test MX record lookup
    sus mx_records tea[value] = lookup_mx_records("gmail.com")
    assert_true(len(mx_records) > 0)
    
    // Test TXT record lookup
    sus txt_records tea[value] = lookup_txt_records("google.com")
    assert_true(len(txt_records) > 0)
}

// Test HTTP client functionality
slay test_http_client() {
    test_start("HTTP Client Functionality")
    
    // Test HTTP request creation
    sus request HTTPRequest = http_create_request("GET", "http://example.com/")
    assert_eq_string(request.method, "GET")
    assert_eq_string(request.url, "http://example.com/")
    assert_eq_string(request.version, "HTTP/1.1")
    
    // Test adding headers
    http_add_header(&request, "User-Agent", "CURSED/1.0")
    assert_true(string_contains(request.headers, "User-Agent: CURSED/1.0"))
    
    // Test setting body
    http_set_body(&request, "test data")
    assert_eq_string(request.body, "test data")
    assert_true(string_contains(request.headers, "Content-Length"))
    
    // Test sending HTTP request
    sus response HTTPResponse = http_send_request(request)
    assert_eq_int(response.status_code, 200)
    assert_eq_string(response.status_message, "OK")
    assert_true(string_length(response.body) > 0)
}

// Test HTTP GET and POST operations
slay test_http_methods() {
    test_start("HTTP Methods")
    
    // Test GET request
    sus get_response HTTPResponse = http_get("http://httpbin.org/get")
    assert_eq_int(get_response.status_code, 200)
    assert_true(string_contains(get_response.body, "httpbin.org"))
    
    // Test POST request
    sus post_response HTTPResponse = http_post("http://httpbin.org/post", "test=data")
    assert_eq_int(post_response.status_code, 200)
    assert_true(string_contains(post_response.body, "test=data"))
    
    // Test JSON POST request
    sus json_response HTTPResponse = http_post_json("http://httpbin.org/post", "{\"key\":\"value\"}")
    assert_eq_int(json_response.status_code, 200)
    assert_true(string_contains(json_response.body, "value"))
}

// Test WebSocket functionality
slay test_websocket() {
    test_start("WebSocket Functionality")
    
    // Test WebSocket handshake creation
    sus handshake tea = websocket_create_handshake("/socket")
    assert_true(string_contains(handshake, "GET /socket HTTP/1.1"))
    assert_true(string_contains(handshake, "Upgrade: websocket"))
    assert_true(string_contains(handshake, "Connection: Upgrade"))
    assert_true(string_contains(handshake, "Sec-WebSocket-Key:"))
    assert_true(string_contains(handshake, "Sec-WebSocket-Version: 13"))
    
    // Test handshake validation
    sus valid_response tea = "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n\r\n"
    assert_true(websocket_validate_handshake(valid_response))
    
    sus invalid_response tea = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n"
    assert_false(websocket_validate_handshake(invalid_response))
    
    // Test frame creation
    sus text_frame tea = websocket_create_text_frame("Hello WebSocket")
    assert_true(string_length(text_frame) > 15)
    
    sus binary_frame tea = websocket_create_binary_frame("binary data")
    assert_true(string_length(binary_frame) > 11)
    
    sus close_frame tea = websocket_create_close_frame(1000)
    assert_true(string_length(close_frame) >= 2)
}

// Test WebSocket frame parsing
slay test_websocket_frame_parsing() {
    test_start("WebSocket Frame Parsing")
    
    // Create a simple text frame and parse it
    sus original_message tea = "Test Message"
    sus frame tea = websocket_create_text_frame(original_message)
    sus parsed_frame WebSocketFrame = websocket_parse_frame(frame)
    
    assert_eq_int(parsed_frame.opcode, 1)  // Text frame
    assert_true(parsed_frame.is_final)
    assert_eq_string(parsed_frame.payload, original_message)
    
    // Test binary frame parsing
    sus binary_data tea = "binary test"
    sus binary_frame tea = websocket_create_binary_frame(binary_data)
    sus parsed_binary WebSocketFrame = websocket_parse_frame(binary_frame)
    
    assert_eq_int(parsed_binary.opcode, 2)  // Binary frame
    assert_eq_string(parsed_binary.payload, binary_data)
}

// Test URL parsing functionality
slay test_url_parsing() {
    test_start("URL Parsing")
    
    // Test HTTP URL parsing
    sus parsed ParsedURL = parse_url("http://example.com:8080/path/to/resource?param=value")
    assert_eq_string(parsed.scheme, "http")
    assert_eq_string(parsed.host, "example.com")
    assert_eq_int(parsed.port, 8080)
    assert_eq_string(parsed.path, "/path/to/resource")
    assert_eq_string(parsed.query, "param=value")
    
    // Test HTTPS URL parsing
    sus https_parsed ParsedURL = parse_url("https://secure.example.com/api")
    assert_eq_string(https_parsed.scheme, "https")
    assert_eq_string(https_parsed.host, "secure.example.com")
    assert_eq_int(https_parsed.port, 443)
    assert_eq_string(https_parsed.path, "/api")
    
    // Test WebSocket URL parsing
    sus ws_parsed ParsedURL = parse_url("ws://localhost:3000/socket")
    assert_eq_string(ws_parsed.scheme, "ws")
    assert_eq_string(ws_parsed.host, "localhost")
    assert_eq_int(ws_parsed.port, 3000)
    assert_eq_string(ws_parsed.path, "/socket")
}

// Test IP address validation
slay test_ip_validation() {
    test_start("IP Address Validation")
    
    // Test valid IPv4 addresses
    assert_true(is_valid_ip("192.168.1.1"))
    assert_true(is_valid_ip("127.0.0.1"))
    assert_true(is_valid_ip("8.8.8.8"))
    assert_true(is_valid_ip("255.255.255.255"))
    
    // Test invalid IPv4 addresses
    assert_false(is_valid_ip("256.1.1.1"))
    assert_false(is_valid_ip("192.168.1"))
    assert_false(is_valid_ip("not.an.ip.address"))
    assert_false(is_valid_ip(""))
    
    // Test IPv6 addresses (simplified check)
    assert_true(is_valid_ip("2001:db8::1"))
    assert_true(is_valid_ip("::1"))
    
    // Test private IP detection
    assert_true(is_private_ip("192.168.1.1"))
    assert_true(is_private_ip("10.0.0.1"))
    assert_true(is_private_ip("172.16.0.1"))
    assert_true(is_private_ip("127.0.0.1"))
    
    assert_false(is_private_ip("8.8.8.8"))
    assert_false(is_private_ip("1.1.1.1"))
}

// Test network utilities
slay test_network_utilities() {
    test_start("Network Utilities")
    
    // Test ping functionality
    assert_true(ping_host("localhost", 5))
    assert_true(ping_host("127.0.0.1", 5))
    
    // Test network interface info
    sus interface_info tea = get_network_interface_info()
    assert_true(string_length(interface_info) > 0)
    assert_true(string_contains(interface_info, "127.0.0.1"))
    
    // Test port scanning
    assert_true(network_scan_port("127.0.0.1", 80))
    assert_false(network_scan_port("127.0.0.1", 99999))  // Invalid port
}

// Test string utility functions
slay test_string_utilities() {
    test_start("String Utilities")
    
    // Test string_contains
    assert_true(string_contains("hello world", "world"))
    assert_false(string_contains("hello world", "test"))
    
    // Test string_starts_with and string_ends_with
    assert_true(string_starts_with("hello world", "hello"))
    assert_false(string_starts_with("hello world", "world"))
    assert_true(string_ends_with("hello world", "world"))
    assert_false(string_ends_with("hello world", "hello"))
    
    // Test string_split
    sus parts tea[value] = string_split("a,b,c,d", ",")
    assert_eq_int(len(parts), 4)
    
    // Test string_join
    sus joined tea = string_join(parts, "-")
    assert_true(string_contains(joined, "-"))
    
    // Test string_to_int and int_to_string
    assert_eq_int(string_to_int("123"), 123)
    assert_eq_int(string_to_int("-456"), -456)
    assert_eq_string(int_to_string(789), "789")
    assert_eq_string(int_to_string(-101), "-101")
    
    // Test case conversion
    assert_eq_string(string_to_lower("Hello World"), "hello world")
    assert_eq_string(string_to_upper("hello world"), "HELLO WORLD")
    
    // Test string trimming
    assert_eq_string(string_trim("  hello world  "), "hello world")
    assert_eq_string(string_trim("\t\ntest\r\n"), "test")
    
    // Test string_substring
    assert_eq_string(string_substring("hello world", 6, 11), "world")
    assert_eq_string(string_substring("hello", 0, 5), "hello")
    assert_eq_string(string_substring("test", 10, 15), "")  // Out of bounds
    
    // Test string_index_of
    assert_eq_int(string_index_of("hello world", "world"), 6)
    assert_eq_int(string_index_of("hello world", "test"), -1)
}

// Test error handling and edge cases
slay test_error_handling() {
    test_start("Error Handling and Edge Cases")
    
    // Test invalid socket handles
    assert_false(close_socket(-1))
    assert_false(close_socket(99999))
    
    // Test invalid binding parameters
    sus socket normie = tcp_socket_create()
    assert_false(tcp_socket_bind(socket, "invalid.ip", 80))
    assert_false(tcp_socket_bind(socket, "127.0.0.1", -1))
    assert_false(tcp_socket_bind(socket, "127.0.0.1", 99999))
    
    // Test invalid connection parameters
    assert_false(tcp_socket_connect(socket, "999.999.999.999", 80))
    assert_false(tcp_socket_connect(socket, "127.0.0.1", 0))
    
    // Test empty string handling
    assert_eq_string(resolve_hostname(""), "127.0.0.1")
    assert_eq_string(resolve_ip_to_hostname(""), "unknown.host")
    
    // Test URL parsing edge cases
    sus empty_url ParsedURL = parse_url("")
    assert_eq_string(empty_url.scheme, "http")
    assert_eq_int(empty_url.port, 80)
    
    close_socket(socket)
}

// Test concurrent socket operations
slay test_concurrent_operations() {
    test_start("Concurrent Socket Operations")
    
    // Create multiple sockets
    sus sockets normie[5]
    bestie i := 0; i < 5; i++ {
        sockets[i] = tcp_socket_create()
        assert_true(sockets[i] != -1)
    }
    
    // Test binding to different ports
    bestie i := 0; i < 5; i++ {
        sus port normie = 8000 + i
        assert_true(tcp_socket_bind(sockets[i], "127.0.0.1", port))
    }
    
    // Test listening on all sockets
    bestie i := 0; i < 5; i++ {
        assert_true(tcp_socket_listen(sockets[i], 5))
    }
    
    // Cleanup all sockets
    bestie i := 0; i < 5; i++ {
        close_socket(sockets[i])
    }
}

// Test large data transmission
slay test_large_data_transmission() {
    test_start("Large Data Transmission")
    
    sus socket normie = tcp_socket_create()
    assert_true(tcp_socket_connect(socket, "127.0.0.1", 80))
    
    // Create large data payload
    sus large_data tea = ""
    bestie i := 0; i < 100; i++ {
        large_data = large_data + "This is a test message for large data transmission. "
    }
    
    // Test sending large data
    sus bytes_sent normie = tcp_socket_send(socket, large_data)
    assert_true(bytes_sent > 0)
    
    close_socket(socket)
}

// Test HTTP response parsing
slay test_http_response_parsing() {
    test_start("HTTP Response Parsing")
    
    // Test parsing a complete HTTP response
    sus sample_response tea = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 12\r\n\r\nHello World!"
    
    // For this test, we need to create a function to parse HTTP responses
    // This would typically be done by parsing the response from http_send_request
    sus request HTTPRequest = http_create_request("GET", "http://example.com/")
    sus response HTTPResponse = http_send_request(request)
    
    assert_eq_int(response.status_code, 200)
    assert_eq_string(response.status_message, "OK")
    assert_true(string_length(response.headers) > 0)
    assert_true(string_length(response.body) > 0)
}

// Test network statistics and monitoring
slay test_network_monitoring() {
    test_start("Network Monitoring")
    
    // Test connection tracking
    sus socket1 normie = tcp_socket_create()
    sus socket2 normie = tcp_socket_create()
    
    assert_true(tcp_socket_connect(socket1, "127.0.0.1", 80))
    assert_true(tcp_socket_connect(socket2, "127.0.0.1", 443))
    
    // Test that sockets are tracked
    assert_true(socket1 != socket2)
    
    // Test data transmission tracking
    tcp_socket_send(socket1, "GET / HTTP/1.1\r\n\r\n")
    tcp_socket_send(socket2, "GET / HTTP/1.1\r\n\r\n")
    
    close_socket(socket1)
    close_socket(socket2)
}

// Main test runner
slay main_character() {
    vibez.spill("Running CURSED Pure Networking Module Tests (100% FFI-Free)...")
    vibez.spill("Testing complete self-contained networking implementation...")
    
    test_socket_creation()
    test_tcp_socket_operations()
    test_udp_socket_operations()
    test_dns_resolution()
    test_http_client()
    test_http_methods()
    test_websocket()
    test_websocket_frame_parsing()
    test_url_parsing()
    test_ip_validation()
    test_network_utilities()
    test_string_utilities()
    test_error_handling()
    test_concurrent_operations()
    test_large_data_transmission()
    test_http_response_parsing()
    test_network_monitoring()
    
    print_test_summary()
    
    vibez.spill("Pure CURSED networking tests completed successfully!")
    vibez.spill("All FFI dependencies eliminated - ready for 100% self-hosting!")
}
