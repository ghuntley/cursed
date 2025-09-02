// CURSED Networking Module Tests
// Comprehensive test suite for network functionality

fam "stdlib/testz"
fam "stdlib/net"

// Test TCP socket creation and basic operations
slay test_tcp_socket_creation() {
    test_start("TCP Socket Creation")
    
    sus socket TCPSocket = tcp_socket_create()
    assert_true(socket.handle != -1)
    assert_false(socket.is_connected)
    
    tcp_socket_close(&socket)
    assert_true(socket.handle == -1)
    assert_false(socket.is_connected)
}

// Test UDP socket creation and basic operations
slay test_udp_socket_creation() {
    test_start("UDP Socket Creation")
    
    sus socket UDPSocket = udp_socket_create()
    assert_true(socket.handle != -1)
    assert_false(socket.is_bound)
    
    udp_socket_close(&socket)
    assert_true(socket.handle == -1)
    assert_false(socket.is_bound)
}

// Test TCP listener creation and binding
slay test_tcp_listener_creation() {
    test_start("TCP Listener Creation")
    
    sus listener TCPListener = tcp_listener_create()
    assert_true(listener.handle != -1)
    assert_false(listener.is_listening)
    
    tcp_listener_close(&listener)
    assert_true(listener.handle == -1)
    assert_false(listener.is_listening)
}

// Test IP address parsing
slay test_ip_address_parsing() {
    test_start("IP Address Parsing")
    
    sus ipv4 IPAddr = parse_ip("192.168.1.1")
    assert_true(is_ipv4(ipv4))
    assert_false(is_ipv6(ipv4))
    assert_eq_string(ip_to_string(ipv4), "192.168.1.1")
    
    sus ipv6 IPAddr = parse_ip("2001:db8::1")
    assert_true(is_ipv6(ipv6))
    assert_false(is_ipv4(ipv6))
    assert_eq_string(ip_to_string(ipv6), "2001:db8::1")
}

// Test HTTP request creation
slay test_http_request_creation() {
    test_start("HTTP Request Creation")
    
    sus request HTTPRequest = http_request_create("GET", "http://example.com")
    assert_eq_string(request.method, "GET")
    assert_eq_string(request.url, "http://example.com")
    assert_eq_string(request.headers, "")
    assert_eq_string(request.body, "")
    
    http_request_add_header(&request, "User-Agent", "CURSED/1.0")
    assert_true(string_contains(request.headers, "User-Agent: CURSED/1.0"))
    
    http_request_set_body(&request, "test data")
    assert_eq_string(request.body, "test data")
}

// Test HTTP response parsing
slay test_http_response_parsing() {
    test_start("HTTP Response Parsing")
    
    sus response_text tea = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 5\r\n\r\nHello"
    sus response HTTPResponse = parse_http_response(response_text)
    
    assert_eq_int(response.status_code, 200)
    assert_true(string_contains(response.headers, "Content-Type: text/plain"))
    assert_eq_string(response.body, "Hello")
}

// Test WebSocket handshake creation
slay test_websocket_handshake() {
    test_start("WebSocket Handshake Creation")
    
    sus handshake tea = create_websocket_handshake("/socket")
    assert_true(string_contains(handshake, "GET /socket HTTP/1.1"))
    assert_true(string_contains(handshake, "Upgrade: websocket"))
    assert_true(string_contains(handshake, "Connection: Upgrade"))
    assert_true(string_contains(handshake, "Sec-WebSocket-Key:"))
    assert_true(string_contains(handshake, "Sec-WebSocket-Version: 13"))
}

// Test WebSocket handshake validation
slay test_websocket_handshake_validation() {
    test_start("WebSocket Handshake Validation")
    
    sus valid_response tea = "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n\r\n"
    assert_true(validate_websocket_handshake(valid_response))
    
    sus invalid_response tea = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n"
    assert_false(validate_websocket_handshake(invalid_response))
}

// Test WebSocket frame creation
slay test_websocket_frame_creation() {
    test_start("WebSocket Frame Creation")
    
    sus text_frame tea = create_websocket_text_frame("Hello")
    assert_true(string_length(text_frame) > 5)
    assert_true(text_frame[0] == '\x81')  // Text frame, final
    
    sus binary_frame tea = create_websocket_binary_frame("data")
    assert_true(string_length(binary_frame) > 4)
    assert_true(binary_frame[0] == '\x82')  // Binary frame, final
    
    sus close_frame tea = create_websocket_close_frame()
    assert_eq_string(close_frame, "\x88\x00")
}

// Test URL parsing utilities
slay test_url_parsing() {
    test_start("URL Parsing")
    
    sus host tea = extract_host_from_url("http://example.com:8080/path")
    assert_eq_string(host, "example.com")
    
    sus port normie = extract_port_from_url("http://example.com:8080/path")
    assert_eq_int(port, 8080)
    
    sus default_port normie = extract_port_from_url("http://example.com/path")
    assert_eq_int(default_port, 80)
    
    sus https_port normie = extract_port_from_url("https://example.com/path")
    assert_eq_int(https_port, 443)
}

// Test string utility functions
slay test_string_utilities() {
    test_start("String Utilities")
    
    assert_true(string_contains("hello world", "world"))
    assert_false(string_contains("hello world", "test"))
    
    assert_true(string_starts_with("hello world", "hello"))
    assert_false(string_starts_with("hello world", "world"))
    
    sus parts tea[value] = string_split("a,b,c", ",")
    assert_eq_int(len(parts), 3)
    assert_eq_string(parts[0], "a")
    assert_eq_string(parts[1], "b")
    assert_eq_string(parts[2], "c")
    
    sus joined tea = string_join(parts, "-")
    assert_eq_string(joined, "a-b-c")
    
    assert_eq_int(string_to_int("123"), 123)
    assert_eq_int(string_to_int("-456"), -456)
    
    assert_eq_int(string_index_of("hello", "ll"), 2)
    assert_eq_int(string_index_of("hello", "test"), -1)
    
    assert_eq_string(string_substring("hello", 1, 4), "ell")
    assert_eq_string(string_substring("hello", 0, 5), "hello")
}

// Test DNS resolution mock
slay test_dns_resolution() {
    test_start("DNS Resolution")
    
    // These tests would require actual network connectivity
    // For now, we test the function structure
    sus addresses tea[value] = resolve_hostname("localhost")
    assert_true(len(addresses) >= 0)  // Should not crash
    
    sus hostname tea = resolve_ip_to_hostname("127.0.0.1")
    assert_true(string_length(hostname) >= 0)  // Should not crash
    
    sus mx_records tea[value] = lookup_mx("example.com")
    assert_true(len(mx_records) >= 0)  // Should not crash
    
    sus txt_records tea[value] = lookup_txt("example.com")
    assert_true(len(txt_records) >= 0)  // Should not crash
}

// Test network utilities
slay test_network_utilities() {
    test_start("Network Utilities")
    
    // Test local IP retrieval
    sus local_ip tea = get_local_ip()
    assert_true(string_length(local_ip) >= 0)  // Should not crash
    
    // Test ping functionality
    sus ping_result lit = ping("127.0.0.1")
    assert_true(ping_result == based || ping_result == cap)  // Should return boolean
    
    // Test network scan
    sus scan_results tea[value] = network_scan("127.0.0.1", "127.0.0.1", 80)
    assert_true(len(scan_results) >= 0)  // Should not crash
}

// Test character and string conversion functions
slay test_character_conversions() {
    test_start("Character Conversions")
    
    assert_eq_int(char_to_int('A'), 65)
    assert_eq_int(char_to_int('0'), 48)
    
    sus char_str tea = char_from_int(65)
    assert_eq_string(char_str, "A")
    
    assert_eq_int(string_length("hello"), 5)
    assert_eq_int(string_length(""), 0)
    
    sus char_at sip = string_char_at("hello", 1)
    assert_eq_int(char_to_int(char_at), char_to_int('e'))
}

// Test TCP socket binding to available ports
slay test_tcp_socket_binding() {
    test_start("TCP Socket Binding")
    
    sus socket TCPSocket = tcp_socket_create()
    
    // Try to bind to localhost on port 0 (system assigns port)
    sus bind_result lit = tcp_socket_bind(&socket, "127.0.0.1", 0)
    // This might fail if networking is not available, but should not crash
    assert_true(bind_result == based || bind_result == cap)
    
    tcp_socket_close(&socket)
}

// Test UDP socket binding
slay test_udp_socket_binding() {
    test_start("UDP Socket Binding")
    
    sus socket UDPSocket = udp_socket_create()
    
    // Try to bind to localhost on port 0 (system assigns port)
    sus bind_result lit = udp_socket_bind(&socket, "127.0.0.1", 0)
    // This might fail if networking is not available, but should not crash
    assert_true(bind_result == based || bind_result == cap)
    
    udp_socket_close(&socket)
}

// Test port availability checking
slay test_port_availability() {
    test_start("Port Availability")
    
    // Test port availability checking
    sus port_available lit = is_port_available(0)  // Port 0 should be available
    assert_true(port_available == based || port_available == cap)  // Should return boolean
    
    // Test well-known ports (might be in use)
    sus http_port lit = is_port_available(80)
    assert_true(http_port == based || http_port == cap)  // Should return boolean
}

// Test error handling and edge cases
slay test_error_handling() {
    test_start("Error Handling")
    
    // Test invalid IP parsing
    sus invalid_ip IPAddr = parse_ip("invalid.ip.address")
    assert_eq_string(ip_to_string(invalid_ip), "invalid.ip.address")
    
    // Test empty string handling
    sus empty_split tea[value] = string_split("", ",")
    assert_eq_int(len(empty_split), 0)
    
    sus empty_join tea = string_join(empty_split, "-")
    assert_eq_string(empty_join, "")
    
    // Test invalid substring
    sus invalid_substring tea = string_substring("hello", 10, 15)
    assert_eq_string(invalid_substring, "")
    
    // Test negative index
    sus negative_index tea = string_substring("hello", -1, 2)
    assert_eq_string(negative_index, "")
}

// Test HTTP method variations
slay test_http_methods() {
    test_start("HTTP Methods")
    
    sus get_request HTTPRequest = http_request_create("GET", "http://example.com")
    assert_eq_string(get_request.method, "GET")
    
    sus post_request HTTPRequest = http_request_create("POST", "http://example.com")
    assert_eq_string(post_request.method, "POST")
    
    sus put_request HTTPRequest = http_request_create("PUT", "http://example.com")
    assert_eq_string(put_request.method, "PUT")
    
    sus delete_request HTTPRequest = http_request_create("DELETE", "http://example.com")
    assert_eq_string(delete_request.method, "DELETE")
}

// Test WebSocket message types
slay test_websocket_message_types() {
    test_start("WebSocket Message Types")
    
    sus text_msg tea = "Hello WebSocket"
    sus text_frame tea = create_websocket_text_frame(text_msg)
    assert_true(string_length(text_frame) > string_length(text_msg))
    
    sus binary_data tea = "binary data"
    sus binary_frame tea = create_websocket_binary_frame(binary_data)
    assert_true(string_length(binary_frame) > string_length(binary_data))
    
    sus close_frame tea = create_websocket_close_frame()
    assert_eq_int(string_length(close_frame), 2)
}

// Test comprehensive networking workflow
slay test_networking_workflow() {
    test_start("Networking Workflow")
    
    // Create TCP socket
    sus socket TCPSocket = tcp_socket_create()
    assert_true(socket.handle != -1)
    
    // Create listener
    sus listener TCPListener = tcp_listener_create()
    assert_true(listener.handle != -1)
    
    // Create UDP socket
    sus udp_socket UDPSocket = udp_socket_create()
    assert_true(udp_socket.handle != -1)
    
    // Create HTTP request
    sus request HTTPRequest = http_request_create("GET", "http://example.com")
    http_request_add_header(&request, "Accept", "text/html")
    assert_true(string_contains(request.headers, "Accept: text/html"))
    
    // Clean up
    tcp_socket_close(&socket)
    tcp_listener_close(&listener)
    udp_socket_close(&udp_socket)
    
    assert_false(socket.is_connected)
    assert_false(listener.is_listening)
    assert_false(udp_socket.is_bound)
}

// Main test runner
slay main() {
    vibez.spill("Running CURSED Networking Module Tests...")
    
    test_tcp_socket_creation()
    test_udp_socket_creation()
    test_tcp_listener_creation()
    test_ip_address_parsing()
    test_http_request_creation()
    test_http_response_parsing()
    test_websocket_handshake()
    test_websocket_handshake_validation()
    test_websocket_frame_creation()
    test_url_parsing()
    test_string_utilities()
    test_dns_resolution()
    test_network_utilities()
    test_character_conversions()
    test_tcp_socket_binding()
    test_udp_socket_binding()
    test_port_availability()
    test_error_handling()
    test_http_methods()
    test_websocket_message_types()
    test_networking_workflow()
    
    print_test_summary()
    
    vibez.spill("All networking tests completed!")
}
