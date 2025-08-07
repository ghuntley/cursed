yeet "httpz"
yeet "tcpz"
yeet "vibez"

fr fr Comprehensive Networking Demo
vibez.spill("🌐 CURSED Networking Demo")
vibez.spill("=======================")

fr fr HTTP GET Demo
vibez.spill("\n📡 HTTP GET Request Demo")
sus get_response HTTPResponse = http_get("http://example.com")
vibez.spill("Status:", get_response.status_code, "-", http_status_text(get_response.status_code))
vibez.spill("Content Type:", http_get_content_type(get_response))
vibez.spill("Response Body:", get_response.body)
vibez.spill("Success:", http_is_success(get_response))

fr fr HTTP POST Demo  
vibez.spill("\n📤 HTTP POST Request Demo")
sus post_response HTTPResponse = http_post("http://httpbin.org/post", "name=cursed&version=1.0")
vibez.spill("Status:", post_response.status_code, "-", http_status_text(post_response.status_code))
vibez.spill("Response Body:", post_response.body)

fr fr HTTP Error Handling Demo
vibez.spill("\n⚠️  HTTP Error Handling Demo")
sus error_response HTTPResponse = http_get("http://example.com/404")
vibez.spill("Status:", error_response.status_code)
vibez.spill("Error:", http_is_error(error_response))
vibez.spill("Body:", error_response.body)

fr fr TCP Connection Demo
vibez.spill("\n🔌 TCP Connection Demo")
sus tcp_result TCPResult = tcp_connect("localhost", 8080)
vibes tcp_result.success {
    vibez.spill("✅ TCP Connection successful!")
    vibez.spill("Remote Address:", tcp_get_remote_addr(tcp_result.connection))
    vibez.spill("Connected:", tcp_is_connected(tcp_result.connection))
    
    fr fr Send data
    sus bytes_sent, send_error = tcp_send(tcp_result.connection, "Hello from CURSED!")
    vibes send_error == "" {
        vibez.spill("✅ Sent", bytes_sent, "bytes")
    } norly {
        vibez.spill("❌ Send error:", send_error)
    }
    
    fr fr Receive data
    sus received_data, recv_error = tcp_receive(tcp_result.connection)
    vibes recv_error == "" {
        vibez.spill("✅ Received:", received_data)
    } norly {
        vibez.spill("❌ Receive error:", recv_error)
    }
    
    fr fr Close connection
    tcp_close(&tcp_result.connection)
    vibez.spill("✅ Connection closed")
} norly {
    vibez.spill("❌ TCP Connection failed:", tcp_result.error)
}

fr fr TCP Server Demo
vibez.spill("\n🏠 TCP Server Demo")
sus server_result TCPServerResult = tcp_listen(9000)
vibes server_result.success {
    vibez.spill("✅ TCP Server listening on port 9000")
    vibez.spill("Server listening:", server_result.server.is_listening)
    
    fr fr Simulate accepting a client
    sus client_result TCPResult = tcp_accept(server_result.server)
    vibes client_result.success {
        vibez.spill("✅ Client connected:", tcp_get_remote_addr(client_result.connection))
        tcp_close(&client_result.connection)
    }
    
    fr fr Close server
    tcp_server_close(&server_result.server)
    vibez.spill("✅ Server closed")
} norly {
    vibez.spill("❌ TCP Server failed:", server_result.error)
}

fr fr Error Handling Demos
vibez.spill("\n🚫 Error Handling Demos")

fr fr Invalid HTTP URL
sus invalid_http HTTPResponse = http_get("")
vibes invalid_http.error != "" {
    vibez.spill("❌ HTTP Error:", invalid_http.error)
}

fr fr Invalid TCP connection
sus invalid_tcp TCPResult = tcp_connect("", 0)
vibes !invalid_tcp.success {
    vibez.spill("❌ TCP Error:", invalid_tcp.error)
}

fr fr Connection Pool Demo
vibez.spill("\n🏊 TCP Connection Pool Demo")
sus pool TCPConnectionPool = tcp_pool_create("localhost", 8080, 3)
vibez.spill("✅ Connection pool created")
vibez.spill("Host:", pool.host)
vibez.spill("Port:", pool.port)
vibez.spill("Max Size:", pool.max_size)
vibez.spill("Active Connections:", pool.active_count)

fr fr Advanced HTTP Request Demo
vibez.spill("\n🔧 Advanced HTTP Request Demo")
sus advanced_request HTTPRequest = http_request_create("POST", "http://api.example.com")
http_request_add_header(&advanced_request, "User-Agent", "CURSED-Client/1.0")
http_request_add_header(&advanced_request, "Authorization", "Bearer token123")
http_request_set_content_type(&advanced_request, "application/json")
http_request_set_body(&advanced_request, "{\"message\":\"Hello from CURSED!\"}")

vibez.spill("✅ Advanced request configured:")
vibez.spill("Method:", advanced_request.method)
vibez.spill("URL:", advanced_request.url)
vibez.spill("Content Type:", advanced_request.content_type)
vibez.spill("Has Headers:", advanced_request.headers != "")

sus advanced_response HTTPResponse = http_send_request(advanced_request)
vibez.spill("Response Status:", advanced_response.status_code)
vibez.spill("Response Body:", advanced_response.body)

fr fr Network Utility Demo
vibez.spill("\n🛠️  Network Utility Demo")
vibez.spill("Valid URL 'https://example.com':", is_valid_url("https://example.com"))
vibez.spill("Valid URL 'not-a-url':", is_valid_url("not-a-url"))
vibez.spill("Valid hostname 'github.com':", is_valid_hostname("github.com"))
vibez.spill("Valid hostname 'bad host':", is_valid_hostname("bad host"))
vibez.spill("Port 8080 available:", is_port_available(8080))
vibez.spill("Port 80 available:", is_port_available(80))

vibez.spill("\n✅ Networking demo completed!")
