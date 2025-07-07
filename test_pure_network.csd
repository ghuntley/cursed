// Test Pure CURSED Network Implementation

yeet "stdlib/network"
yeet "testz"

// Test basic networking functionality
slay test_tcp_socket() {
    test_start("TCP Socket Creation")
    
    sus socket normie = tcp_create()
    assert_true(socket > 0, "TCP socket should be created with positive ID")
    
    print_test_summary()
}

slay test_tcp_connect() {
    test_start("TCP Socket Connect")
    
    sus socket normie = tcp_create()
    sus result normie = tcp_connect(socket, "127.0.0.1", 80)
    
    // Should succeed for localhost
    assert_eq_int(result, 0, "TCP connect to localhost should succeed")
    
    tcp_close(socket)
    print_test_summary()
}

slay test_udp_socket() {
    test_start("UDP Socket Operations")
    
    sus socket normie = udp_create()
    assert_true(socket > 0, "UDP socket should be created with positive ID")
    
    sus bind_result normie = udp_bind(socket, "127.0.0.1", 12345)
    assert_eq_int(bind_result, 0, "UDP bind should succeed")
    
    udp_close(socket)
    print_test_summary()
}

slay test_dns_resolution() {
    test_start("DNS Resolution")
    
    sus ip tea = resolve_hostname("localhost")
    assert_eq_string(ip, "127.0.0.1", "localhost should resolve to 127.0.0.1")
    
    sus hostname tea = resolve_ip("127.0.0.1")
    assert_eq_string(hostname, "localhost", "127.0.0.1 should resolve to localhost")
    
    print_test_summary()
}

slay test_http_simulation() {
    test_start("HTTP Request Simulation")
    
    sus response tea = http_send("GET", "http://example.com/", "", "")
    assert_true(string_length(response) > 0, "HTTP response should not be empty")
    
    print_test_summary()
}

slay test_network_utilities() {
    test_start("Network Utilities")
    
    sus local_ip tea = get_local_ip()
    assert_eq_string(local_ip, "127.0.0.1", "Local IP should be 127.0.0.1")
    
    sus ping_result lit = ping("localhost")
    assert_true(ping_result, "Ping localhost should succeed")
    
    print_test_summary()
}

// Run all tests
test_tcp_socket()
test_tcp_connect()
test_udp_socket()
test_dns_resolution()
test_http_simulation()
test_network_utilities()

vibez.spill("✅ All pure CURSED network tests completed!")
