vibe main {
    // Test DNS resolution
    let dns_result = network_dns_resolve("google.com");
    io_println("DNS resolution for google.com:");
    io_println(dns_result);
    
    // Test HTTP GET request
    let http_response = network_http_get("http://httpbin.org/get");
    io_println("HTTP GET response:");
    io_println(http_response);
    
    // Test TCP connection
    let tcp_connection = network_tcp_connect("google.com", 80);
    if tcp_connection > 0 {
        io_println("TCP connection successful");
        network_tcp_close(tcp_connection);
    } else {
        io_println("TCP connection failed");
    }
}
