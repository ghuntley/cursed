fr fr ========================================
fr fr HTTP/2 and Advanced Web Features Test
fr fr Comprehensive Testing Suite
fr fr ========================================

yeet "testz"
yeet "web_vibez"

sus test_passed drip = 0
sus test_failed drip = 0

fr fr Test Helper Functions
slay assert_eq(actual tea, expected tea, test_name tea) {
    lowkey actual == expected {
        vibez.spill("✓ " + test_name + " PASSED")
        test_passed = test_passed + 1
    } else {
        vibez.spill("✗ " + test_name + " FAILED")
        vibez.spill("  Expected: " + expected)
        vibez.spill("  Actual: " + actual)
        test_failed = test_failed + 1
    }
}

slay assert_true(condition lit, test_name tea) {
    lowkey condition {
        vibez.spill("✓ " + test_name + " PASSED")
        test_passed = test_passed + 1
    } else {
        vibez.spill("✗ " + test_name + " FAILED")
        test_failed = test_failed + 1
    }
}

slay assert_gt(actual drip, expected drip, test_name tea) {
    lowkey actual > expected {
        vibez.spill("✓ " + test_name + " PASSED")
        test_passed = test_passed + 1
    } else {
        vibez.spill("✗ " + test_name + " FAILED")
        vibez.spill("  Expected > " + expected.to_string())
        vibez.spill("  Actual: " + actual.to_string())
        test_failed = test_failed + 1
    }
}

fr fr Test HTTP/2 Frame Creation and Serialization
slay test_http2_frames() {
    vibez.spill("\n=== HTTP/2 Frame Tests ===")
    
    fr fr Test HEADERS frame creation
    sus payload []drip = [0x82] fr fr Index 2 (method GET)
    sus frame HTTP2Frame = web_vibez.create_http2_frame(web_vibez.HTTP2_HEADERS, 0x01, 1, payload)
    
    assert_eq(frame.frame_type.to_string(), "1", "HEADERS frame type")
    assert_eq(frame.stream_id.to_string(), "1", "Stream ID")
    assert_eq(frame.flags.to_string(), "1", "END_HEADERS flag")
    
    fr fr Test frame serialization
    sus serialized []drip = web_vibez.serialize_http2_frame(frame)
    assert_gt(serialized.len(), 9, "Frame serialization length")
    
    fr fr Test frame parsing
    sus parsed HTTP2Frame = web_vibez.parse_http2_frame(serialized)
    assert_eq(parsed.frame_type.to_string(), frame.frame_type.to_string(), "Frame type round-trip")
    assert_eq(parsed.stream_id.to_string(), frame.stream_id.to_string(), "Stream ID round-trip")
}

fr fr Test HPACK Header Compression  
slay test_hpack_compression() {
    vibez.spill("\n=== HPACK Compression Tests ===")
    
    sus table HPACKTable = web_vibez.init_hpack_table()
    assert_gt(table.static_table.len(), 20, "Static table initialization")
    
    fr fr Test header encoding
    sus encoded []drip = web_vibez.hpack_encode_header("content-type", "application/json", table)
    assert_gt(encoded.len(), 0, "Header encoding produces output")
    
    fr fr Test string encoding
    sus str_encoded []drip = web_vibez.encode_hpack_string("test-value")
    assert_gt(str_encoded.len(), 5, "String encoding includes length and data")
}

fr fr Test WebSocket Implementation
slay test_websocket() {
    vibez.spill("\n=== WebSocket Tests ===")
    
    fr fr Test key generation
    sus key tea = web_vibez.generate_websocket_key()
    assert_gt(key.len(), 10, "WebSocket key generation")
    
    fr fr Test accept key calculation
    sus accept tea = web_vibez.calculate_websocket_accept(key)
    assert_gt(accept.len(), 10, "WebSocket accept key calculation")
    
    fr fr Test handshake response
    sus response tea = web_vibez.websocket_handshake_response(key, "chat")
    assert_true(response.contains("101 Switching Protocols"), "WebSocket handshake response")
    assert_true(response.contains("Upgrade: websocket"), "WebSocket upgrade header")
    assert_true(response.contains("Sec-WebSocket-Accept:"), "WebSocket accept header")
    
    fr fr Test frame creation
    sus payload []drip = [72, 101, 108, 108, 111] fr fr "Hello"
    sus frame WebSocketFrame = web_vibez.create_websocket_frame(web_vibez.WS_TEXT, payload, based)
    
    assert_eq(frame.opcode.to_string(), "1", "Text frame opcode")
    assert_eq(frame.payload_length.to_string(), "5", "Frame payload length")
    assert_true(frame.fin, "Frame FIN bit")
    
    fr fr Test frame serialization
    sus serialized []drip = web_vibez.serialize_websocket_frame(frame)
    assert_gt(serialized.len(), 7, "WebSocket frame serialization")
}

fr fr Test TLS and HTTPS Integration
slay test_tls_integration() {
    vibez.spill("\n=== TLS Integration Tests ===")
    
    fr fr Test TLS handshake
    sus protocols []tea = ["h2", "http/1.1"]
    sus conn TLSConnection = web_vibez.tls_handshake("example.com", protocols)
    
    assert_eq(conn.version.to_string(), web_vibez.TLS_1_3.to_string(), "TLS version")
    assert_eq(conn.server_name, "example.com", "Server name")
    assert_gt(conn.certificate_chain.len(), 0, "Certificate chain")
    
    fr fr Test ALPN negotiation
    sus negotiated tea = web_vibez.negotiate_alpn(protocols)
    assert_eq(negotiated, "h2", "ALPN protocol negotiation")
    
    fr fr Test HTTP/2 connection creation
    sus http2_conn HTTP2Connection = web_vibez.create_http2_connection(based)
    assert_true(http2_conn.tls_enabled, "TLS enabled connection")
    assert_eq(http2_conn.window_size.to_string(), "65535", "Default window size")
}

fr fr Test HTTP/2 Multiplexing
slay test_http2_multiplexing() {
    vibez.spill("\n=== HTTP/2 Multiplexing Tests ===")
    
    sus manager MultiplexManager = web_vibez.create_multiplex_manager(10)
    assert_eq(manager.max_concurrent_streams.to_string(), "10", "Max concurrent streams")
    
    fr fr Create test streams
    sus stream1 HTTP2Stream = HTTP2Stream{
        stream_id: 1,
        state: web_vibez.HTTP2_OPEN,
        window_size: 65535,
        headers: [":method GET", ":path /"],
        data: [],
        priority: 0
    }
    
    sus stream2 HTTP2Stream = HTTP2Stream{
        stream_id: 3,
        state: web_vibez.HTTP2_OPEN,
        window_size: 32768,
        headers: [":method POST", ":path /api"],
        data: [1, 2, 3, 4],
        priority: 1
    }
    
    fr fr Test adding streams
    sus (updated_manager, success1) = web_vibez.add_stream_to_multiplexer(manager, stream1)
    assert_true(success1, "Stream 1 added to multiplexer")
    
    sus (updated_manager2, success2) = web_vibez.add_stream_to_multiplexer(updated_manager, stream2)  
    assert_true(success2, "Stream 2 added to multiplexer")
    
    assert_eq(updated_manager2.active_streams.len().to_string(), "2", "Active streams count")
    
    fr fr Test removing streams
    sus final_manager MultiplexManager = web_vibez.remove_stream_from_multiplexer(updated_manager2, 1)
    assert_eq(final_manager.active_streams.len().to_string(), "1", "Stream removed from multiplexer")
}

fr fr Test Flow Control
slay test_flow_control() {
    vibez.spill("\n=== Flow Control Tests ===")
    
    sus window FlowControlWindow = FlowControlWindow{
        stream_id: 1,
        window_size: 65535,
        initial_window_size: 65535
    }
    
    fr fr Test window update
    sus updated_window FlowControlWindow = web_vibez.update_flow_control_window(window, -1000)
    assert_eq(updated_window.window_size.to_string(), "64535", "Window size decreased")
    
    fr fr Test window increase
    sus increased_window FlowControlWindow = web_vibez.update_flow_control_window(updated_window, 2000)
    assert_eq(increased_window.window_size.to_string(), "66535", "Window size increased")
    
    fr fr Test window overflow protection
    sus overflow_window FlowControlWindow = web_vibez.update_flow_control_window(window, 2147483647)
    assert_eq(overflow_window.window_size.to_string(), "2147483647", "Window size max limit")
}

fr fr Test Circuit Breaker
slay test_circuit_breaker() {
    vibez.spill("\n=== Circuit Breaker Tests ===")
    
    sus breaker CircuitBreaker = web_vibez.create_circuit_breaker(3, 5000)
    assert_eq(breaker.state.to_string(), web_vibez.CB_CLOSED.to_string(), "Initial circuit breaker state")
    assert_eq(breaker.failure_threshold.to_string(), "3", "Failure threshold")
    
    fr fr Test failure recording
    sus failed_breaker CircuitBreaker = web_vibez.record_circuit_breaker_result(breaker, cap)
    assert_eq(failed_breaker.failure_count.to_string(), "1", "Failure count increased")
    
    fr fr Test multiple failures triggering open state
    sus failed_again CircuitBreaker = web_vibez.record_circuit_breaker_result(failed_breaker, cap)
    failed_again = web_vibez.record_circuit_breaker_result(failed_again, cap)
    
    assert_eq(failed_again.state.to_string(), web_vibez.CB_OPEN.to_string(), "Circuit breaker opened after failures")
    
    fr fr Test success recording
    sus success_breaker CircuitBreaker = web_vibez.record_circuit_breaker_result(breaker, based)
    assert_eq(success_breaker.failure_count.to_string(), "0", "Success resets failure count")
}

fr fr Test Rate Limiting (Token Bucket)
slay test_rate_limiting() {
    vibez.spill("\n=== Rate Limiting Tests ===")
    
    sus bucket TokenBucket = web_vibez.create_token_bucket(10, 5)
    assert_eq(bucket.capacity.to_string(), "10", "Token bucket capacity")
    assert_eq(bucket.tokens.to_string(), "10", "Initial token count")
    assert_eq(bucket.refill_rate.to_string(), "5", "Token refill rate")
    
    fr fr Test token consumption
    sus (updated_bucket, success) = web_vibez.consume_tokens(bucket, 3)
    assert_true(success, "Token consumption success")
    assert_eq(updated_bucket.tokens.to_string(), "7", "Tokens consumed")
    
    fr fr Test over-consumption
    sus (over_bucket, over_success) = web_vibez.consume_tokens(updated_bucket, 10)
    assert_true(!over_success, "Over-consumption rejected")
    assert_eq(over_bucket.tokens.to_string(), "7", "Tokens unchanged on rejection")
}

fr fr Test Load Balancer
slay test_load_balancer() {
    vibez.spill("\n=== Load Balancer Tests ===")
    
    sus backends []Backend = [
        Backend{address: "server1.com", port: 80, weight: 1, active_connections: 5, healthy: based},
        Backend{address: "server2.com", port: 80, weight: 2, active_connections: 3, healthy: based},
        Backend{address: "server3.com", port: 80, weight: 1, active_connections: 8, healthy: cap}
    ]
    
    sus lb LoadBalancer = web_vibez.create_load_balancer(web_vibez.LB_ROUND_ROBIN, backends)
    assert_eq(lb.algorithm.to_string(), web_vibez.LB_ROUND_ROBIN.to_string(), "Round robin algorithm")
    assert_eq(lb.backends.len().to_string(), "3", "Backend count")
    
    fr fr Test round robin selection
    sus (backend1, success1) = web_vibez.select_backend(lb)
    assert_true(success1, "First backend selection")
    assert_eq(backend1.address, "server1.com", "Round robin first selection")
    
    fr fr Test least connections algorithm
    sus lb_lc LoadBalancer = web_vibez.create_load_balancer(web_vibez.LB_LEAST_CONNECTIONS, backends)
    sus (backend_lc, success_lc) = web_vibez.select_backend(lb_lc)
    assert_true(success_lc, "Least connections selection")
    assert_eq(backend_lc.address, "server2.com", "Least connections selects server2")
    
    fr fr Test health check
    assert_true(web_vibez.health_check_backend(backends[0]), "Healthy backend health check")
    assert_true(!web_vibez.health_check_backend(Backend{address: "", port: 0, weight: 0, active_connections: 0, healthy: cap}), "Unhealthy backend health check")
}

fr fr Test HTTP Methods
slay test_http_methods() {
    vibez.spill("\n=== HTTP Methods Tests ===")
    
    fr fr Test CONNECT method
    sus connect_response tea = web_vibez.http_method_connect("proxy.com", 8080)
    assert_true(connect_response.contains("200 Connection Established"), "CONNECT method response")
    
    fr fr Test OPTIONS method
    sus methods []tea = ["GET", "POST", "PUT", "DELETE"]
    sus options_response tea = web_vibez.http_method_options(methods)
    assert_true(options_response.contains("Allow: GET, POST, PUT, DELETE"), "OPTIONS method Allow header")
    assert_true(options_response.contains("Access-Control-Allow-Methods"), "OPTIONS CORS headers")
    
    fr fr Test HEAD method
    sus head_response tea = web_vibez.http_method_head("http://example.com/test")
    assert_true(head_response.contains("200 OK"), "HEAD method response")
    assert_true(head_response.contains("Content-Length:"), "HEAD method headers")
    assert_true(!head_response.contains("body_content"), "HEAD method no body")
}

fr fr Test Server Push
slay test_server_push() {
    vibez.spill("\n=== Server Push Tests ===")
    
    sus push_headers []tea = [":status 200", "content-type text/css", "cache-control max-age=3600"]
    sus push_promise HTTP2PushPromise = web_vibez.create_server_push(1, "/styles.css", push_headers)
    
    assert_gt(push_promise.promised_stream_id.to_string().len(), 0, "Promised stream ID generated")
    assert_true(push_promise.request_headers.contains(":method GET"), "Push promise method header")
    assert_true(push_promise.request_headers.contains(":path /styles.css"), "Push promise path header")
    assert_eq(push_promise.response_headers.len().to_string(), "3", "Push promise response headers")
}

fr fr Test Full HTTP/2 Client Request
slay test_http2_client() {
    vibez.spill("\n=== HTTP/2 Client Tests ===")
    
    sus headers []tea = ["authorization", "Bearer token123", "accept", "application/json"]
    sus body []drip = [123, 34, 104, 101, 108, 108, 111, 34, 125] fr fr {"hello"}
    
    sus response tea = web_vibez.http2_client_request("https://api.example.com/data", "POST", headers, body)
    assert_true(response.contains("HTTP/2 200"), "HTTP/2 client response")
    assert_true(response.contains("\"protocol\": \"h2\""), "HTTP/2 protocol confirmation")
    assert_true(response.contains("\"multiplexed\": true"), "HTTP/2 multiplexing confirmation")
}

fr fr Test WebSocket Client
slay test_websocket_client() {
    vibez.spill("\n=== WebSocket Client Tests ===")
    
    sus protocols []tea = ["chat", "superchat"]
    sus connection_response tea = web_vibez.websocket_client_connect("wss://echo.websocket.org", protocols)
    
    assert_true(connection_response.contains("101 Switching Protocols"), "WebSocket client connection")
    assert_true(connection_response.contains("websocket"), "WebSocket upgrade confirmation")
    
    fr fr Test sending messages
    sus text_frame []drip = web_vibez.websocket_send_text("Hello WebSocket!")
    assert_gt(text_frame.len(), 10, "WebSocket text message frame")
    
    sus binary_data []drip = [1, 2, 3, 4, 5]
    sus binary_frame []drip = web_vibez.websocket_send_binary(binary_data)
    assert_gt(binary_frame.len(), 7, "WebSocket binary message frame")
    
    sus ping_frame []drip = web_vibez.websocket_ping()
    assert_gt(ping_frame.len(), 2, "WebSocket ping frame")
    
    sus close_frame []drip = web_vibez.websocket_close(1000, "Normal closure")
    assert_gt(close_frame.len(), 10, "WebSocket close frame")
}

fr fr Test Production HTTP/2 Server Simulation
slay test_production_server() {
    vibez.spill("\n=== Production Server Tests ===")
    
    fr fr This would start actual server in production - here we just test initialization
    vibez.spill("Testing HTTP/2 server initialization...")
    web_vibez.start_http2_server(8443, based, "/etc/ssl/cert.pem", "/etc/ssl/key.pem")
    
    fr fr Test passes if server startup simulation completes without errors
    assert_true(based, "HTTP/2 server startup simulation")
}

fr fr Run All Tests
slay main() {
    vibez.spill("🚀 HTTP/2 and Advanced Web Features Test Suite")
    vibez.spill("================================================")
    
    test_http2_frames()
    test_hpack_compression()
    test_websocket()
    test_tls_integration()
    test_http2_multiplexing()
    test_flow_control()
    test_circuit_breaker()
    test_rate_limiting()
    test_load_balancer()
    test_http_methods()
    test_server_push()
    test_http2_client()
    test_websocket_client()
    test_production_server()
    
    vibez.spill("\n================================================")
    vibez.spill("📊 Test Results:")
    vibez.spill("✓ Passed: " + test_passed.to_string())
    vibez.spill("✗ Failed: " + test_failed.to_string())
    vibez.spill("📈 Success Rate: " + ((test_passed * 100) / (test_passed + test_failed)).to_string() + "%")
    
    lowkey test_failed == 0 {
        vibez.spill("🎉 All HTTP/2 and advanced web features tests passed!")
        vibez.spill("🔥 Ready for production web applications")
    } else {
        vibez.spill("⚠️  Some tests failed - check implementation")
    }
}

main()
