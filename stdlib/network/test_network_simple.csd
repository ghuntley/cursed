// Simple Network Test - Basic functionality check
yeet "stdlib/network"

slay main_character() {
    vibez.spill("🧪 Testing Network Module - Basic Functions")
    
    // Test TCP socket creation
    sus tcp_socket normie = tcp_create()
    vibez.spill("✅ TCP Socket created with ID: " + int_to_string(tcp_socket))
    
    // Test UDP socket creation
    sus udp_socket normie = udp_create()
    vibez.spill("✅ UDP Socket created with ID: " + int_to_string(udp_socket))
    
    // Test DNS resolution
    sus resolved_ip tea = resolve_hostname("localhost")
    vibez.spill("✅ DNS Resolution - localhost -> " + resolved_ip)
    
    // Test reverse DNS
    sus resolved_hostname tea = resolve_ip("127.0.0.1")
    vibez.spill("✅ Reverse DNS - 127.0.0.1 -> " + resolved_hostname)
    
    // Test local IP
    sus local_ip tea = get_local_ip()
    vibez.spill("✅ Local IP: " + local_ip)
    
    // Test ping simulation
    sus ping_result lit = ping("localhost")
    if ping_result {
        vibez.spill("✅ Ping to localhost succeeded")
    } else {
        vibez.spill("❌ Ping to localhost failed")
    }
    
    // Test TCP bind
    sus bind_result normie = tcp_bind(tcp_socket, "127.0.0.1", 8080)
    if bind_result == 0 {
        vibez.spill("✅ TCP bind to localhost:8080 succeeded")
    } else {
        vibez.spill("❌ TCP bind to localhost:8080 failed")
    }
    
    // Test TCP connect
    sus connect_result normie = tcp_connect(udp_socket, "127.0.0.1", 80)
    if connect_result == 0 {
        vibez.spill("✅ TCP connect to localhost:80 succeeded")
    } else {
        vibez.spill("❌ TCP connect to localhost:80 failed")
    }
    
    // Test HTTP simulation
    sus http_response tea = http_send("GET", "http://example.com/", "", "")
    if string_length(http_response) > 0 {
        vibez.spill("✅ HTTP request simulation returned response")
        vibez.spill("Response preview: " + string_substring(http_response, 0, 50) + "...")
    } else {
        vibez.spill("❌ HTTP request simulation failed")
    }
    
    // Clean up
    tcp_close(tcp_socket)
    udp_close(udp_socket)
    
    vibez.spill("🎉 Network Module Basic Test Complete!")
}

main()
