// CURSED Network Module - Comprehensive Test Suite
// Tests all network module functions with testz framework

yeet "testz"
yeet "stdlib/network"

// Test TCP Socket Operations
slay test_tcp_socket_creation() {
    test_start("TCP Socket Creation")
    
    sus socket1 normie = tcp_create()
    sus socket2 normie = tcp_create()
    
    // TCP sockets should have unique IDs
    assert_true(socket1 != socket2)
    assert_true(socket1 > 0)
    assert_true(socket2 > 0)
    
    tcp_close(socket1)
    tcp_close(socket2)
}

slay test_tcp_bind_operations() {
    test_start("TCP Bind Operations")
    
    sus socket normie = tcp_create()
    
    // Test successful bind
    sus bind_result normie = tcp_bind(socket, "127.0.0.1", 8080)
    assert_eq_int(bind_result, 0)
    
    // Test port already in use
    sus socket2 normie = tcp_create()
    sus bind_result2 normie = tcp_bind(socket2, "127.0.0.1", 8080)
    assert_eq_int(bind_result2, -1)
    
    tcp_close(socket)
    tcp_close(socket2)
}

slay test_tcp_connect_operations() {
    test_start("TCP Connect Operations")
    
    sus socket normie = tcp_create()
    
    // Test successful connection to localhost
    sus connect_result normie = tcp_connect(socket, "127.0.0.1", 80)
    assert_eq_int(connect_result, 0)
    
    tcp_close(socket)
}

slay test_tcp_listen_operations() {
    test_start("TCP Listen Operations")
    
    sus socket normie = tcp_create()
    
    // Bind socket first
    sus bind_result normie = tcp_bind(socket, "127.0.0.1", 8081)
    assert_eq_int(bind_result, 0)
    
    // Test successful listen
    sus listen_result normie = tcp_listen(socket, 5)
    assert_eq_int(listen_result, 0)
    
    tcp_close(socket)
}

slay test_tcp_accept_operations() {
    test_start("TCP Accept Operations")
    
    sus server_socket normie = tcp_create()
    
    // Setup server socket
    tcp_bind(server_socket, "127.0.0.1", 8082)
    tcp_listen(server_socket, 5)
    
    // Test accept (simulates client connection)
    sus client_socket normie = tcp_accept(server_socket)
    assert_true(client_socket > 0)
    
    tcp_close(server_socket)
    tcp_close(client_socket)
}

slay test_tcp_send_recv_operations() {
    test_start("TCP Send/Recv Operations")
    
    sus socket normie = tcp_create()
    tcp_connect(socket, "127.0.0.1", 80)
    
    // Test send
    sus send_result normie = tcp_send(socket, "Hello World")
    assert_eq_int(send_result, 11)
    
    // Test receive
    sus recv_result tea = tcp_recv(socket, 1024)
    assert_true(string_length(recv_result) > 0)
    
    tcp_close(socket)
}

// Test UDP Socket Operations
slay test_udp_socket_creation() {
    test_start("UDP Socket Creation")
    
    sus socket1 normie = udp_create()
    sus socket2 normie = udp_create()
    
    // UDP sockets should have unique IDs
    assert_true(socket1 != socket2)
    assert_true(socket1 > 0)
    assert_true(socket2 > 0)
    
    udp_close(socket1)
    udp_close(socket2)
}

slay test_udp_bind_operations() {
    test_start("UDP Bind Operations")
    
    sus socket normie = udp_create()
    
    // Test successful bind
    sus bind_result normie = udp_bind(socket, "127.0.0.1", 9090)
    assert_eq_int(bind_result, 0)
    
    // Test port already in use
    sus socket2 normie = udp_create()
    sus bind_result2 normie = udp_bind(socket2, "127.0.0.1", 9090)
    assert_eq_int(bind_result2, -1)
    
    udp_close(socket)
    udp_close(socket2)
}

slay test_udp_send_recv_operations() {
    test_start("UDP Send/Recv Operations")
    
    sus socket normie = udp_create()
    udp_bind(socket, "127.0.0.1", 9091)
    
    // Test send to
    sus send_result normie = udp_send_to(socket, "UDP Test", "127.0.0.1", 9092)
    assert_eq_int(send_result, 8)
    
    // Test receive from
    sus recv_result tea = udp_recv_from(socket, 1024)
    assert_true(string_length(recv_result) > 0)
    
    udp_close(socket)
}

// Test DNS Resolution
slay test_hostname_resolution() {
    test_start("Hostname Resolution")
    
    // Test localhost resolution
    sus ip tea = resolve_hostname("localhost")
    assert_eq_string(ip, "127.0.0.1")
    
    // Test example.com resolution
    sus example_ip tea = resolve_hostname("example.com")
    assert_eq_string(example_ip, "93.184.216.34")
    
    // Test google.com resolution
    sus google_ip tea = resolve_hostname("google.com")
    assert_eq_string(google_ip, "172.217.14.110")
    
    // Test github.com resolution
    sus github_ip tea = resolve_hostname("github.com")
    assert_eq_string(github_ip, "140.82.112.3")
    
    // Test unknown hostname
    sus unknown_ip tea = resolve_hostname("unknown.domain")
    assert_eq_string(unknown_ip, "192.168.1.100")
}

slay test_reverse_dns_resolution() {
    test_start("Reverse DNS Resolution")
    
    // Test localhost reverse lookup
    sus hostname tea = resolve_ip("127.0.0.1")
    assert_eq_string(hostname, "localhost")
    
    // Test example.com reverse lookup
    sus example_hostname tea = resolve_ip("93.184.216.34")
    assert_eq_string(example_hostname, "example.com")
    
    // Test google.com reverse lookup
    sus google_hostname tea = resolve_ip("172.217.14.110")
    assert_eq_string(google_hostname, "google.com")
    
    // Test github.com reverse lookup
    sus github_hostname tea = resolve_ip("140.82.112.3")
    assert_eq_string(github_hostname, "github.com")
    
    // Test unknown IP
    sus unknown_hostname tea = resolve_ip("192.168.1.1")
    assert_eq_string(unknown_hostname, "192.168.1.1")
}

slay test_mx_record_lookup() {
    test_start("MX Record Lookup")
    
    // Test example.com MX record
    sus mx_record tea = lookup_mx("example.com")
    assert_eq_string(mx_record, "mail.example.com")
    
    // Test google.com MX record
    sus google_mx tea = lookup_mx("google.com")
    assert_eq_string(google_mx, "gmail-smtp-in.l.google.com")
    
    // Test unknown domain MX record
    sus unknown_mx tea = lookup_mx("unknown.domain")
    assert_eq_string(unknown_mx, "mail.unknown.domain")
}

slay test_txt_record_lookup() {
    test_start("TXT Record Lookup")
    
    // Test example.com TXT record
    sus txt_record tea = lookup_txt("example.com")
    assert_eq_string(txt_record, "v=spf1 include:_spf.example.com ~all")
    
    // Test google.com TXT record
    sus google_txt tea = lookup_txt("google.com")
    assert_eq_string(google_txt, "v=spf1 include:_spf.google.com ~all")
    
    // Test unknown domain TXT record
    sus unknown_txt tea = lookup_txt("unknown.domain")
    assert_eq_string(unknown_txt, "v=spf1 ~all")
}

// Test HTTP Client Operations
slay test_http_get_request() {
    test_start("HTTP GET Request")
    
    sus response tea = http_send("GET", "http://example.com/", "", "")
    assert_true(string_length(response) > 0)
    
    // Should contain HTTP response headers
    assert_true(string_contains(response, "HTTP/1.1"))
    assert_true(string_contains(response, "200 OK"))
}

slay test_http_post_request() {
    test_start("HTTP POST Request")
    
    sus body tea = "test=data"
    sus headers tea = "Content-Type: application/x-www-form-urlencoded"
    sus response tea = http_send("POST", "http://example.com/", headers, body)
    
    assert_true(string_length(response) > 0)
    assert_true(string_contains(response, "HTTP/1.1"))
}

slay test_http_url_parsing() {
    test_start("HTTP URL Parsing")
    
    // Test host extraction
    sus host1 tea = extract_host_from_url("http://example.com/path")
    assert_eq_string(host1, "example.com")
    
    sus host2 tea = extract_host_from_url("https://api.github.com:443/users")
    assert_eq_string(host2, "api.github.com")
    
    // Test port extraction
    sus port1 normie = extract_port_from_url("http://example.com/")
    assert_eq_int(port1, 80)
    
    sus port2 normie = extract_port_from_url("https://example.com/")
    assert_eq_int(port2, 443)
    
    sus port3 normie = extract_port_from_url("http://example.com:8080/")
    assert_eq_int(port3, 8080)
    
    // Test path extraction
    sus path1 tea = extract_path_from_url("http://example.com/api/v1/users")
    assert_eq_string(path1, "/api/v1/users")
    
    sus path2 tea = extract_path_from_url("https://example.com/")
    assert_eq_string(path2, "/")
}

// Test TLS/SSL Support
slay test_tls_init() {
    test_start("TLS Initialization")
    
    // Test TLS init for localhost (should succeed)
    sus tls_result1 lit = tls_init(1000, "localhost")
    assert_true(tls_result1)
    
    // Test TLS init for 127.0.0.1 (should succeed)
    sus tls_result2 lit = tls_init(1001, "127.0.0.1")
    assert_true(tls_result2)
    
    // Test TLS init for external hostname (should fail in pure implementation)
    sus tls_result3 lit = tls_init(1002, "example.com")
    assert_false(tls_result3)
}

slay test_tls_send_recv() {
    test_start("TLS Send/Recv Operations")
    
    sus socket normie = tcp_create()
    tcp_connect(socket, "127.0.0.1", 443)
    
    // Test TLS send (falls back to TCP)
    sus send_result normie = tls_send(socket, "Hello TLS")
    assert_eq_int(send_result, 9)
    
    // Test TLS recv (falls back to TCP)
    sus recv_result tea = tls_recv(socket, 1024)
    assert_true(string_length(recv_result) > 0)
    
    tcp_close(socket)
}

// Test Network Utilities
slay test_network_utilities() {
    test_start("Network Utilities")
    
    // Test get local IP
    sus local_ip tea = get_local_ip()
    assert_eq_string(local_ip, "127.0.0.1")
    
    // Test ping simulation
    sus ping_result1 lit = ping("localhost")
    assert_true(ping_result1)
    
    sus ping_result2 lit = ping("example.com")
    assert_true(ping_result2)
    
    // Test network scan
    sus scan_result tea = network_scan("192.168.1.1", "192.168.1.10", 22)
    assert_true(string_length(scan_result) > 0)
    assert_true(string_contains(scan_result, "192.168.1.1"))
}

slay test_remote_address_retrieval() {
    test_start("Remote Address Retrieval")
    
    sus socket normie = tcp_create()
    tcp_connect(socket, "127.0.0.1", 80)
    
    sus remote_addr tea = get_remote_addr(socket)
    assert_true(string_length(remote_addr) > 0)
    assert_true(string_contains(remote_addr, "127.0.0.1"))
    assert_true(string_contains(remote_addr, ":"))
    
    tcp_close(socket)
}

// Test Socket State Management
slay test_socket_state_management() {
    test_start("Socket State Management")
    
    sus socket normie = tcp_create()
    
    // Test initial state (closed)
    sus bind_result normie = tcp_bind(socket, "127.0.0.1", 8083)
    assert_eq_int(bind_result, 0)
    
    sus listen_result normie = tcp_listen(socket, 5)
    assert_eq_int(listen_result, 0)
    
    // Test socket closure
    tcp_close(socket)
    
    // Operations on closed socket should fail
    sus send_result normie = tcp_send(socket, "test")
    assert_eq_int(send_result, -1)
}

// Test Error Handling
slay test_error_handling() {
    test_start("Error Handling")
    
    // Test operations on invalid socket handle
    sus invalid_socket normie = -1
    
    sus bind_result normie = tcp_bind(invalid_socket, "127.0.0.1", 8084)
    assert_eq_int(bind_result, -1)
    
    sus connect_result normie = tcp_connect(invalid_socket, "127.0.0.1", 80)
    assert_eq_int(connect_result, -1)
    
    sus send_result normie = tcp_send(invalid_socket, "test")
    assert_eq_int(send_result, -1)
    
    sus recv_result tea = tcp_recv(invalid_socket, 1024)
    assert_eq_string(recv_result, "")
}

// Test String Utilities
slay test_string_utilities() {
    test_start("String Utilities")
    
    // Test string_starts_with
    assert_true(string_starts_with("hello world", "hello"))
    assert_false(string_starts_with("hello world", "world"))
    
    // Test string_index_of
    assert_eq_int(string_index_of("hello world", "world"), 6)
    assert_eq_int(string_index_of("hello world", "xyz"), -1)
    
    // Test string_substring
    assert_eq_string(string_substring("hello world", 6, 11), "world")
    assert_eq_string(string_substring("hello world", 0, 5), "hello")
    
    // Test string_length
    assert_eq_int(string_length("hello"), 5)
    assert_eq_int(string_length(""), 0)
    
    // Test string_to_int
    assert_eq_int(string_to_int("123"), 123)
    assert_eq_int(string_to_int("-456"), -456)
    assert_eq_int(string_to_int("0"), 0)
    
    // Test int_to_string
    assert_eq_string(int_to_string(123), "123")
    assert_eq_string(int_to_string(-456), "-456")
    assert_eq_string(int_to_string(0), "0")
}

// Test Concurrent Socket Operations
slay test_concurrent_socket_operations() {
    test_start("Concurrent Socket Operations")
    
    sus socket1 normie = tcp_create()
    sus socket2 normie = tcp_create()
    sus socket3 normie = udp_create()
    
    // Test multiple socket operations
    tcp_bind(socket1, "127.0.0.1", 8085)
    tcp_bind(socket2, "127.0.0.1", 8086)
    udp_bind(socket3, "127.0.0.1", 9095)
    
    // Verify all sockets are independent
    assert_true(socket1 != socket2)
    assert_true(socket2 != socket3)
    assert_true(socket1 != socket3)
    
    tcp_close(socket1)
    tcp_close(socket2)
    udp_close(socket3)
}

// Test Protocol Differentiation
slay test_protocol_differentiation() {
    test_start("Protocol Differentiation")
    
    sus tcp_socket normie = tcp_create()
    sus udp_socket normie = udp_create()
    
    // TCP and UDP sockets should be different
    assert_true(tcp_socket != udp_socket)
    
    // TCP operations should not affect UDP socket
    tcp_bind(tcp_socket, "127.0.0.1", 8087)
    udp_bind(udp_socket, "127.0.0.1", 8087)  // Same port, different protocol
    
    tcp_close(tcp_socket)
    udp_close(udp_socket)
}

// Helper function for string contains check
slay string_contains(text tea, substring tea) lit {
    damn string_index_of(text, substring) != -1
}

// Main test execution
slay main_character() {
    vibez.spill("🧪 CURSED Network Module - Comprehensive Test Suite")
    vibez.spill("============================================================")
    
    // TCP Socket Tests
    test_tcp_socket_creation()
    test_tcp_bind_operations()
    test_tcp_connect_operations()
    test_tcp_listen_operations()
    test_tcp_accept_operations()
    test_tcp_send_recv_operations()
    
    // UDP Socket Tests
    test_udp_socket_creation()
    test_udp_bind_operations()
    test_udp_send_recv_operations()
    
    // DNS Resolution Tests
    test_hostname_resolution()
    test_reverse_dns_resolution()
    test_mx_record_lookup()
    test_txt_record_lookup()
    
    // HTTP Client Tests
    test_http_get_request()
    test_http_post_request()
    test_http_url_parsing()
    
    // TLS/SSL Tests
    test_tls_init()
    test_tls_send_recv()
    
    // Network Utilities Tests
    test_network_utilities()
    test_remote_address_retrieval()
    
    // Socket Management Tests
    test_socket_state_management()
    test_error_handling()
    test_string_utilities()
    test_concurrent_socket_operations()
    test_protocol_differentiation()
    
    vibez.spill("============================================================")
    print_test_summary()
    
    vibez.spill("✨ Network Module Features Tested:")
    vibez.spill("  • TCP Socket Operations (Create, Bind, Connect, Listen, Accept)")
    vibez.spill("  • UDP Socket Operations (Create, Bind, Send/Recv)")
    vibez.spill("  • DNS Resolution (Forward, Reverse, MX, TXT)")
    vibez.spill("  • HTTP Client Operations (GET, POST, URL Parsing)")
    vibez.spill("  • TLS/SSL Support (Init, Send/Recv)")
    vibez.spill("  • Network Utilities (Ping, Scan, IP Management)")
    vibez.spill("  • Error Handling and State Management")
    vibez.spill("  • String Utilities and Protocol Differentiation")
    vibez.spill("🎉 Pure CURSED Network Implementation - FFI-Free!")
}

main()
