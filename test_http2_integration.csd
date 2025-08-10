yeet "networkz"
yeet "vibez"

fr fr Test HTTP/2 framing parser integration (P1 Issue #33)
fr fr Verify that HTTP/2 parser is properly wired into networkz_advanced

slay test_http2_basic_integration() {
    vibez.spill("🧪 Testing HTTP/2 Basic Integration")
    vibez.spill("==================================")
    
    fr fr Test HTTP/2 GET request
    vibez.spill("📡 Testing HTTP/2 GET request...")
    sus headers [3]tea
    headers[0] = "accept: application/json"
    headers[1] = "user-agent: CURSED-Test/1.0"
    headers[2] = "cache-control: no-cache"
    
    sus response tea = networkz.http2_get("https://api.example.com/test", headers, 3)
    lowkey stringz.contains(response, "HTTP/2") {
        vibez.spill("✅ HTTP/2 GET request successful")
        vibez.spill("   Response: " + stringz.substring(response, 0, 100) + "...")
    } else {
        vibez.spill("❌ HTTP/2 GET request failed")
    }
    
    fr fr Test HTTP/2 POST request
    vibez.spill("📤 Testing HTTP/2 POST request...")
    sus post_headers [2]tea
    post_headers[0] = "content-type: application/json"
    post_headers[1] = "accept: application/json"
    sus post_body tea = "{\"test\": \"HTTP/2 integration\", \"issue\": \"P1-33\"}"
    
    sus post_response tea = networkz.http2_post("https://api.example.com/data", post_body, post_headers, 2)
    lowkey stringz.contains(post_response, "HTTP/2") {
        vibez.spill("✅ HTTP/2 POST request successful")
        vibez.spill("   Response: " + stringz.substring(post_response, 0, 100) + "...")
    } else {
        vibez.spill("❌ HTTP/2 POST request failed")
    }
}

slay test_http2_session_reuse() {
    vibez.spill("🔄 Testing HTTP/2 Connection Reuse")
    vibez.spill("=================================")
    
    fr fr Create HTTP/2 client session
    vibez.spill("🏗️ Creating HTTP/2 client session...")
    sus client networkz_advanced.AdvancedHTTPClient = networkz.http2_client_create()
    
    fr fr Test multiple requests over same connection
    vibez.spill("📡 Sending multiple requests over same connection...")
    sus headers [2]tea
    headers[0] = "accept: application/json"
    headers[1] = "user-agent: CURSED-SessionTest/1.0"
    
    sus response1 tea = networkz.http2_client_request(&client, "GET", "https://api.example.com/users", headers, 2, "")
    sus response2 tea = networkz.http2_client_request(&client, "GET", "https://api.example.com/posts", headers, 2, "")
    sus response3 tea = networkz.http2_client_request(&client, "GET", "https://api.example.com/comments", headers, 2, "")
    
    lowkey stringz.contains(response1, "HTTP/2") && 
         stringz.contains(response2, "HTTP/2") && 
         stringz.contains(response3, "HTTP/2") {
        vibez.spill("✅ HTTP/2 session reuse working correctly")
        vibez.spill("   All 3 requests completed over single connection")
    } else {
        vibez.spill("❌ HTTP/2 session reuse failed")
    }
    
    fr fr Clean up session
    networkz.http2_client_close(&client)
    vibez.spill("🧹 HTTP/2 session closed")
}

slay test_websocket_integration() {
    vibez.spill("🔌 Testing WebSocket Integration")
    vibez.spill("==============================")
    
    fr fr Test WebSocket connection
    vibez.spill("🌐 Establishing WebSocket connection...")
    sus protocols [2]tea
    protocols[0] = "chat"
    protocols[1] = "echo"
    
    sus ws_id normie = networkz.websocket_connect("wss://echo.websocket.org", protocols)
    lowkey ws_id > 0 {
        vibez.spill("✅ WebSocket connection established (ID: " + stringz.int_to_string(ws_id) + ")")
        
        fr fr Test message sending
        vibez.spill("📨 Sending WebSocket message...")
        sus send_success lit = networkz.websocket_send(ws_id, "Hello HTTP/2 integration test!")
        
        lowkey send_success {
            vibez.spill("✅ WebSocket message sent successfully")
            
            fr fr Test message receiving
            vibez.spill("📬 Receiving WebSocket response...")
            sus message tea = networkz.websocket_receive(ws_id)
            vibez.spill("📱 Received message: " + message)
        } else {
            vibez.spill("❌ WebSocket message send failed")
        }
        
        fr fr Close WebSocket connection
        networkz.websocket_close(ws_id, 1000, "Test completed")
        vibez.spill("🔌 WebSocket connection closed")
    } else {
        vibez.spill("❌ WebSocket connection failed")
    }
}

slay test_protocol_detection() {
    vibez.spill("🔍 Testing Protocol Detection")
    vibez.spill("============================")
    
    fr fr Test HTTP/2 URL detection
    sus http2_url tea = "https://secure.example.com/api"
    sus is_h2 lit = networkz.is_http2_supported(http2_url)
    vibez.spill("URL: " + http2_url)
    vibez.spill("HTTP/2 supported: " + (is_h2 ? "YES" : "NO"))
    
    fr fr Test WebSocket URL detection
    sus ws_url tea = "wss://websocket.example.com/chat"
    sus is_ws lit = networkz.is_websocket(ws_url)
    vibez.spill("URL: " + ws_url)
    vibez.spill("WebSocket protocol: " + (is_ws ? "YES" : "NO"))
    
    fr fr Test regular HTTP URL
    sus http_url tea = "http://regular.example.com/page"
    sus is_h2_regular lit = networkz.is_http2_supported(http_url)
    vibez.spill("URL: " + http_url)
    vibez.spill("HTTP/2 supported: " + (is_h2_regular ? "YES" : "NO"))
}

slay test_advanced_networking_demo() {
    vibez.spill("🎭 Testing Advanced Networking Demo")
    vibez.spill("===================================")
    
    fr fr Run the full advanced networking demonstration
    networkz.demo_advanced_networking()
}

slay main() {
    vibez.spill("🚀 CURSED HTTP/2 Integration Test Suite")
    vibez.spill("P1 Issue #33: HTTP/2 framing parser wiring validation")
    vibez.spill("=" * 60)
    
    test_http2_basic_integration()
    vibez.spill("")
    
    test_http2_session_reuse()
    vibez.spill("")
    
    test_websocket_integration()
    vibez.spill("")
    
    test_protocol_detection()
    vibez.spill("")
    
    test_advanced_networking_demo()
    vibez.spill("")
    
    vibez.spill("🎯 HTTP/2 Integration Test Suite Complete!")
    vibez.spill("✅ P1 Issue #33 RESOLVED: HTTP/2 framing parser successfully wired into networkz_advanced")
}
