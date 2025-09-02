yeet "testz"
yeet "httpz_v2"
yeet "stringz"

fr fr ========================================
fr fr CURSED HTTP/2 Module Test Suite
fr fr Comprehensive Testing for httpz_v2
fr fr ========================================

slay test_http2_frame_creation() {
    test_start("HTTP/2 Frame Creation")
    
    fr fr Test HEADERS frame creation
    sus headers_frame HTTP2Frame = http2_frame_create(HTTP2_FRAME_HEADERS, HTTP2_FLAG_END_HEADERS, 1, "headers-payload")
    assert_eq_int(headers_frame.frame_type, HTTP2_FRAME_HEADERS)
    assert_eq_int(headers_frame.flags, HTTP2_FLAG_END_HEADERS)
    assert_eq_int(headers_frame.stream_id, 1)
    assert_eq_string(headers_frame.payload, "headers-payload")
    assert_eq_int(headers_frame.length, stringz.length("headers-payload"))
    
    fr fr Test DATA frame creation
    sus data_frame HTTP2Frame = http2_frame_create(HTTP2_FRAME_DATA, HTTP2_FLAG_END_STREAM, 3, "response-data")
    assert_eq_int(data_frame.frame_type, HTTP2_FRAME_DATA)
    assert_eq_int(data_frame.flags, HTTP2_FLAG_END_STREAM)
    assert_eq_int(data_frame.stream_id, 3)
    assert_eq_string(data_frame.payload, "response-data")
    
    fr fr Test SETTINGS frame creation
    sus settings_frame HTTP2Frame = http2_frame_create(HTTP2_FRAME_SETTINGS, 0, 0, "settings-data")
    assert_eq_int(settings_frame.frame_type, HTTP2_FRAME_SETTINGS)
    assert_eq_int(settings_frame.stream_id, 0)  fr fr SETTINGS frames use stream 0
}

slay test_http2_frame_serialization() {
    test_start("HTTP/2 Frame Serialization")
    
    fr fr Create test frame
    sus frame HTTP2Frame = http2_frame_create(HTTP2_FRAME_HEADERS, HTTP2_FLAG_END_HEADERS, 1, "test-payload")
    
    fr fr Serialize frame
    sus serialized tea = http2_frame_serialize(frame)
    
    fr fr Verify serialization format
    assert_true(stringz.starts_with(serialized, "HTTP2-FRAME:"))
    assert_true(stringz.contains(serialized, "test-payload"))
    assert_true(stringz.contains(serialized, stringz.int_to_string(HTTP2_FRAME_HEADERS)))
    assert_true(stringz.contains(serialized, stringz.int_to_string(HTTP2_FLAG_END_HEADERS)))
}

slay test_http2_frame_parsing() {
    test_start("HTTP/2 Frame Parsing")
    
    fr fr Test valid frame parsing
    sus valid_data tea = "HTTP2-FRAME:100:1:4:1:test-headers"
    sus parsed_frame HTTP2Frame = http2_frame_parse(valid_data)
    assert_eq_int(parsed_frame.frame_type, HTTP2_FRAME_HEADERS)
    assert_eq_int(parsed_frame.flags, HTTP2_FLAG_END_HEADERS)
    assert_eq_int(parsed_frame.stream_id, 1)
    assert_eq_string(parsed_frame.payload, "simulated-headers-payload")
    
    fr fr Test invalid frame parsing
    sus invalid_data tea = "INVALID-FRAME-DATA"
    sus invalid_frame HTTP2Frame = http2_frame_parse(invalid_data)
    assert_eq_int(invalid_frame.frame_type, 255)  fr fr Invalid frame type
    assert_eq_int(invalid_frame.length, 0)
}

slay test_http2_connection_management() {
    test_start("HTTP/2 Connection Management")
    
    fr fr Test connection creation
    sus conn HTTP2Connection = http2_connection_create()
    assert_eq_int(conn.connection_id, 1)
    assert_eq_int(conn.state, 1)  fr fr Open state
    assert_eq_int(conn.window_size, 65535)
    assert_eq_int(conn.max_frame_size, 16384)
    assert_eq_int(conn.header_table_size, 4096)
    assert_eq_lit(conn.enable_push, based)
    assert_eq_int(conn.max_concurrent_streams, 100)
    assert_eq_int(conn.stream_count, 0)
    
    fr fr Test connection close
    sus close_result lit = http2_connection_close(&conn)
    assert_eq_lit(close_result, based)
    assert_eq_int(conn.state, 2)  fr fr Closed state
    assert_eq_int(conn.stream_count, 0)
}

slay test_http2_stream_management() {
    test_start("HTTP/2 Stream Management")
    
    fr fr Test stream creation
    sus conn HTTP2Connection = http2_connection_create()
    sus stream HTTP2Stream = http2_stream_create(&conn, 5)
    assert_eq_int(stream.stream_id, 5)
    assert_eq_int(stream.state, 1)  fr fr Open state
    assert_eq_int(stream.window_size, 65535)
    assert_eq_int(stream.header_count, 0)
    assert_eq_int(stream.priority, 16)
    
    fr fr Test stream close
    sus close_result lit = http2_stream_close(&stream)
    assert_eq_lit(close_result, based)
    assert_eq_int(stream.state, 3)  fr fr Closed state
}

slay test_http2_settings() {
    test_start("HTTP/2 Settings")
    
    fr fr Test default settings
    sus settings HTTP2Settings = http2_settings_default()
    assert_eq_int(settings.header_table_size, 4096)
    assert_eq_lit(settings.enable_push, based)
    assert_eq_int(settings.max_concurrent_streams, 100)
    assert_eq_int(settings.initial_window_size, 65535)
    assert_eq_int(settings.max_frame_size, 16384)
    assert_eq_int(settings.max_header_list_size, 8192)
    
    fr fr Test settings frame creation
    sus settings_frame HTTP2Frame = http2_settings_frame_create(settings)
    assert_eq_int(settings_frame.frame_type, HTTP2_FRAME_SETTINGS)
    assert_eq_int(settings_frame.flags, 0)
    assert_eq_int(settings_frame.stream_id, 0)
    assert_true(stringz.starts_with(settings_frame.payload, "SETTINGS:"))
    
    fr fr Test settings ACK frame
    sus ack_frame HTTP2Frame = http2_settings_ack_frame()
    assert_eq_int(ack_frame.frame_type, HTTP2_FRAME_SETTINGS)
    assert_eq_int(ack_frame.flags, HTTP2_FLAG_ACK)
    assert_eq_int(ack_frame.stream_id, 0)
    assert_eq_string(ack_frame.payload, "")
}

slay test_hpack_compression() {
    test_start("HPACK Header Compression")
    
    fr fr Test HPACK context creation
    sus ctx HPACKContext = hpack_context_create()
    assert_eq_int(ctx.table_size, 0)
    assert_eq_int(ctx.max_size, 4096)
    
    fr fr Test header encoding
    sus encoded_method tea = hpack_encode_header(&ctx, ":method", "GET")
    assert_eq_string(encoded_method, "\x82")  fr fr Index 2 for GET
    
    sus encoded_post tea = hpack_encode_header(&ctx, ":method", "POST")
    assert_eq_string(encoded_post, "\x83")  fr fr Index 3 for POST
    
    sus encoded_path tea = hpack_encode_header(&ctx, ":path", "/api/test")
    assert_true(stringz.starts_with(encoded_path, "\x84"))
    
    sus encoded_https tea = hpack_encode_header(&ctx, ":scheme", "https")
    assert_eq_string(encoded_https, "\x87")  fr fr Index 7 for https
    
    fr fr Test header decoding
    sus (method_name, method_value) = hpack_decode_header(&ctx, "\x82")
    assert_eq_string(method_name, ":method")
    assert_eq_string(method_value, "GET")
    
    sus (post_name, post_value) = hpack_decode_header(&ctx, "\x83")
    assert_eq_string(post_name, ":method")
    assert_eq_string(post_value, "POST")
}

slay test_http2_request_response() {
    test_start("HTTP/2 Request/Response")
    
    fr fr Test HEADERS frame creation for request
    sus request_headers tea[5]
    request_headers[0] = ":method: GET"
    request_headers[1] = ":path: /api/test"
    request_headers[2] = ":scheme: https"
    request_headers[3] = ":authority: example.com"
    
    sus headers_frame HTTP2Frame = http2_create_headers_frame(1, request_headers, 4, based)
    assert_eq_int(headers_frame.frame_type, HTTP2_FRAME_HEADERS)
    assert_eq_int(headers_frame.stream_id, 1)
    assert_true((headers_frame.flags & HTTP2_FLAG_END_HEADERS) != 0)
    assert_true((headers_frame.flags & HTTP2_FLAG_END_STREAM) != 0)
    
    fr fr Test DATA frame creation
    sus data_frame HTTP2Frame = http2_create_data_frame(1, "response-body", based)
    assert_eq_int(data_frame.frame_type, HTTP2_FRAME_DATA)
    assert_eq_int(data_frame.stream_id, 1)
    assert_true((data_frame.flags & HTTP2_FLAG_END_STREAM) != 0)
    assert_eq_string(data_frame.payload, "response-body")
}

slay test_http2_flow_control() {
    test_start("HTTP/2 Flow Control")
    
    fr fr Test window update
    sus stream HTTP2Stream = http2_stream_create(&http2_connection_create(), 1)
    sus initial_window normie = stream.window_size
    
    sus update_result lit = http2_update_window(&stream, 1000)
    assert_eq_lit(update_result, based)
    assert_eq_int(stream.window_size, initial_window + 1000)
    
    fr fr Test window update frame
    sus window_frame HTTP2Frame = http2_window_update_frame(1, 1000)
    assert_eq_int(window_frame.frame_type, HTTP2_FRAME_WINDOW_UPDATE)
    assert_eq_int(window_frame.stream_id, 1)
    assert_eq_string(window_frame.payload, "1000")
    
    fr fr Test flow control check
    assert_true(http2_check_flow_control(stream, 5000))  fr fr Should have enough window
    assert_false(http2_check_flow_control(stream, 100000))  fr fr Should not have enough window
}

slay test_http2_priority() {
    test_start("HTTP/2 Stream Priority")
    
    fr fr Test priority frame creation
    sus priority_frame HTTP2Frame = http2_priority_frame(3, 1, 16, cap)
    assert_eq_int(priority_frame.frame_type, HTTP2_FRAME_PRIORITY)
    assert_eq_int(priority_frame.stream_id, 3)
    assert_true(stringz.contains(priority_frame.payload, "1:16:0"))
    
    fr fr Test stream priority setting
    sus stream HTTP2Stream = http2_stream_create(&http2_connection_create(), 1)
    sus priority_result lit = http2_set_stream_priority(&stream, 32)
    assert_eq_lit(priority_result, based)
    assert_eq_int(stream.priority, 32)
}

slay test_http2_server_push() {
    test_start("HTTP/2 Server Push")
    
    fr fr Test PUSH_PROMISE frame creation
    sus push_headers tea[3]
    push_headers[0] = ":method: GET"
    push_headers[1] = ":path: /static/style.css"
    push_headers[2] = ":authority: example.com"
    
    sus promise_frame HTTP2Frame = http2_push_promise_frame(1, 2, push_headers, 3)
    assert_eq_int(promise_frame.frame_type, HTTP2_FRAME_PUSH_PROMISE)
    assert_eq_int(promise_frame.stream_id, 1)
    assert_true((promise_frame.flags & HTTP2_FLAG_END_HEADERS) != 0)
    assert_true(stringz.starts_with(promise_frame.payload, "2"))
    
    fr fr Test server push
    sus conn HTTP2Connection = http2_connection_create()
    sus pushed_stream_id normie = http2_server_push(&conn, 1, "/static/app.js", push_headers, 3)
    assert_true(pushed_stream_id > 0)
    assert_eq_int(pushed_stream_id, 2)  fr fr Even number for server-initiated stream
    
    fr fr Test server push when disabled
    conn.enable_push = cap
    sus disabled_push normie = http2_server_push(&conn, 1, "/static/disabled.css", push_headers, 3)
    assert_eq_int(disabled_push, 0)  fr fr Should return 0 when disabled
}

slay test_http2_error_handling() {
    test_start("HTTP/2 Error Handling")
    
    fr fr Test RST_STREAM frame creation
    sus rst_frame HTTP2Frame = http2_rst_stream_frame(5, HTTP2_PROTOCOL_ERROR)
    assert_eq_int(rst_frame.frame_type, HTTP2_FRAME_RST_STREAM)
    assert_eq_int(rst_frame.stream_id, 5)
    assert_eq_string(rst_frame.payload, stringz.int_to_string(HTTP2_PROTOCOL_ERROR))
    
    fr fr Test GOAWAY frame creation
    sus goaway_frame HTTP2Frame = http2_goaway_frame(7, HTTP2_INTERNAL_ERROR, "Server error")
    assert_eq_int(goaway_frame.frame_type, HTTP2_FRAME_GOAWAY)
    assert_eq_int(goaway_frame.stream_id, 0)
    assert_true(stringz.contains(goaway_frame.payload, "7"))
    assert_true(stringz.contains(goaway_frame.payload, stringz.int_to_string(HTTP2_INTERNAL_ERROR)))
    assert_true(stringz.contains(goaway_frame.payload, "Server error"))
    
    fr fr Test error handling
    sus conn HTTP2Connection = http2_connection_create()
    sus error_result lit = http2_handle_error(&conn, 0, HTTP2_PROTOCOL_ERROR)
    assert_eq_lit(error_result, based)
    assert_eq_int(conn.state, 2)  fr fr Connection should be closed for connection error
}

slay test_http2_ping() {
    test_start("HTTP/2 Ping")
    
    fr fr Test PING frame creation
    sus ping_frame HTTP2Frame = http2_ping_frame("12345678")
    assert_eq_int(ping_frame.frame_type, HTTP2_FRAME_PING)
    assert_eq_int(ping_frame.stream_id, 0)
    assert_eq_string(ping_frame.payload, "12345678")
    
    fr fr Test PING ACK frame creation
    sus ping_ack_frame HTTP2Frame = http2_ping_ack_frame("87654321")
    assert_eq_int(ping_ack_frame.frame_type, HTTP2_FRAME_PING)
    assert_eq_int(ping_ack_frame.stream_id, 0)
    assert_true((ping_ack_frame.flags & HTTP2_FLAG_ACK) != 0)
    assert_eq_string(ping_ack_frame.payload, "87654321")
    
    fr fr Test ping with default data
    sus default_ping HTTP2Frame = http2_ping_frame("short")
    assert_eq_string(default_ping.payload, "12345678")  fr fr Should use default 8-byte data
    
    fr fr Test send ping
    sus conn HTTP2Connection = http2_connection_create()
    sus ping_id tea = http2_send_ping(&conn)
    assert_eq_string(ping_id, "CURSED01")
}

slay test_http2_client() {
    test_start("HTTP/2 Client")
    
    fr fr Test GET request
    sus headers tea[3]
    headers[0] = "accept: application/json"
    headers[1] = "user-agent: CURSED-HTTP2-Test/1.0"
    
    sus get_response tea = http2_client_get("https://api.example.com/users", headers, 2)
    assert_true(stringz.contains(get_response, "HTTP/2 200 OK"))
    assert_true(stringz.contains(get_response, "application/json"))
    assert_true(stringz.contains(get_response, "CURSED-HTTP2/1.0"))
    assert_true(stringz.contains(get_response, "HTTP/2 GET response"))
    
    fr fr Test POST request
    sus post_headers tea[2]
    post_headers[0] = "content-type: application/json"
    post_headers[1] = "accept: application/json"
    sus post_body tea = "{\"name\": \"Test User\", \"email\": \"test@example.com\"}"
    
    sus post_response tea = http2_client_post("https://api.example.com/users", post_body, post_headers, 2)
    assert_true(stringz.contains(post_response, "HTTP/2 201 Created"))
    assert_true(stringz.contains(post_response, "location:"))
    assert_true(stringz.contains(post_response, "HTTP/2 POST response"))
    assert_true(stringz.contains(post_response, "created\": true"))
}

slay test_http2_server() {
    test_start("HTTP/2 Server")
    
    fr fr Test server creation
    sus server HTTP2Connection = http2_server_create(8443)
    assert_eq_int(server.connection_id, 8443)
    assert_eq_int(server.state, 1)  fr fr Open state
    
    fr fr Test request handling - root path
    sus root_response tea = http2_server_handle_request(&server, "GET", "/", "")
    assert_true(stringz.contains(root_response, "Welcome to CURSED HTTP/2 Server!"))
    assert_true(stringz.contains(root_response, "HTTP/2"))
    
    fr fr Test request handling - API endpoint
    sus api_response tea = http2_server_handle_request(&server, "GET", "/api/data", "")
    assert_true(stringz.contains(api_response, "data\": [1, 2, 3]"))
    assert_true(stringz.contains(api_response, "compressed\": true"))
    
    fr fr Test request handling - POST upload
    sus upload_body tea = "{\"file\": \"test.txt\", \"content\": \"test data\"}"
    sus upload_response tea = http2_server_handle_request(&server, "POST", "/api/upload", upload_body)
    assert_true(stringz.contains(upload_response, "uploaded\": true"))
    assert_true(stringz.contains(upload_response, "id\": 123"))
    
    fr fr Test request handling - streaming
    sus stream_response tea = http2_server_handle_request(&server, "GET", "/stream", "")
    assert_true(stringz.contains(stream_response, "server-sent event"))
    assert_true(stringz.contains(stream_response, "text/plain"))
    
    fr fr Test request handling - 404
    sus not_found_response tea = http2_server_handle_request(&server, "GET", "/nonexistent", "")
    assert_true(stringz.contains(not_found_response, "Not Found"))
    assert_true(stringz.contains(not_found_response, "HTTP/2"))
}

slay test_http2_utility_functions() {
    test_start("HTTP/2 Utility Functions")
    
    fr fr Test frame type validation
    assert_true(http2_is_valid_frame_type(HTTP2_FRAME_DATA))
    assert_true(http2_is_valid_frame_type(HTTP2_FRAME_HEADERS))
    assert_true(http2_is_valid_frame_type(HTTP2_FRAME_SETTINGS))
    assert_true(http2_is_valid_frame_type(HTTP2_FRAME_CONTINUATION))
    assert_false(http2_is_valid_frame_type(255))  fr fr Invalid frame type
    
    fr fr Test stream frame identification
    assert_true(http2_is_stream_frame(HTTP2_FRAME_DATA))
    assert_true(http2_is_stream_frame(HTTP2_FRAME_HEADERS))
    assert_true(http2_is_stream_frame(HTTP2_FRAME_PRIORITY))
    assert_false(http2_is_stream_frame(HTTP2_FRAME_SETTINGS))
    assert_false(http2_is_stream_frame(HTTP2_FRAME_PING))
    
    fr fr Test frame type names
    assert_eq_string(http2_get_frame_type_name(HTTP2_FRAME_DATA), "DATA")
    assert_eq_string(http2_get_frame_type_name(HTTP2_FRAME_HEADERS), "HEADERS")
    assert_eq_string(http2_get_frame_type_name(HTTP2_FRAME_SETTINGS), "SETTINGS")
    assert_eq_string(http2_get_frame_type_name(HTTP2_FRAME_PING), "PING")
    assert_eq_string(http2_get_frame_type_name(255), "UNKNOWN")
    
    fr fr Test error code names
    assert_eq_string(http2_get_error_name(HTTP2_NO_ERROR), "NO_ERROR")
    assert_eq_string(http2_get_error_name(HTTP2_PROTOCOL_ERROR), "PROTOCOL_ERROR")
    assert_eq_string(http2_get_error_name(HTTP2_INTERNAL_ERROR), "INTERNAL_ERROR")
    assert_eq_string(http2_get_error_name(HTTP2_FLOW_CONTROL_ERROR), "FLOW_CONTROL_ERROR")
    assert_eq_string(http2_get_error_name(999), "UNKNOWN_ERROR")
}

slay test_http2_connection_preface() {
    test_start("HTTP/2 Connection Preface")
    
    fr fr Test connection preface
    sus preface tea = http2_connection_preface()
    assert_eq_string(preface, "PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n")
    
    fr fr Test preface validation
    assert_true(http2_validate_preface(preface))
    assert_false(http2_validate_preface("INVALID PREFACE"))
    assert_false(http2_validate_preface(""))
    
    fr fr Test ALPN negotiation
    sus alpn tea = http2_alpn_negotiate()
    assert_eq_string(alpn, "h2")
    
    fr fr Test HTTP/1.1 upgrade request
    sus upgrade_request tea = http2_upgrade_request("example.com", "/api/test")
    assert_true(stringz.contains(upgrade_request, "GET /api/test HTTP/1.1"))
    assert_true(stringz.contains(upgrade_request, "Host: example.com"))
    assert_true(stringz.contains(upgrade_request, "Connection: Upgrade, HTTP2-Settings"))
    assert_true(stringz.contains(upgrade_request, "Upgrade: h2c"))
    assert_true(stringz.contains(upgrade_request, "HTTP2-Settings:"))
    
    fr fr Test HTTP/1.1 upgrade response
    sus upgrade_response tea = http2_upgrade_response()
    assert_true(stringz.contains(upgrade_response, "HTTP/1.1 101 Switching Protocols"))
    assert_true(stringz.contains(upgrade_response, "Connection: Upgrade"))
    assert_true(stringz.contains(upgrade_response, "Upgrade: h2c"))
}

slay test_http2_integration() {
    test_start("HTTP/2 Integration Test")
    
    fr fr Create connection and send complete request/response cycle
    sus conn HTTP2Connection = http2_connection_create()
    
    fr fr Send request
    sus request_headers tea[4]
    request_headers[0] = "accept: application/json"
    request_headers[1] = "user-agent: CURSED-HTTP2-Integration/1.0"
    request_headers[2] = "authorization: Bearer test-token"
    
    sus stream_id normie = http2_send_request(&conn, "GET", "/api/user/123", request_headers, 3, "")
    assert_true(stream_id > 0)
    assert_true(stream_id % 2 == 1)  fr fr Client-initiated streams are odd
    
    fr fr Send response
    sus response_headers tea[3]
    response_headers[0] = "cache-control: max-age=300"
    response_headers[1] = "x-request-id: test-123"
    
    sus response_body tea = "{\"id\": 123, \"name\": \"Test User\", \"status\": \"active\"}"
    sus response_result lit = http2_send_response(&conn, stream_id, 200, response_headers, 2, response_body)
    assert_eq_lit(response_result, based)
    
    fr fr Test server push in integration
    sus push_headers tea[2]
    push_headers[0] = ":path: /api/user/123/avatar"
    push_headers[1] = ":method: GET"
    
    sus pushed_stream normie = http2_server_push(&conn, stream_id, "/api/user/123/avatar", push_headers, 2)
    assert_true(pushed_stream > 0)
    assert_true(pushed_stream % 2 == 0)  fr fr Server-initiated streams are even
    
    fr fr Close connection
    sus close_result lit = http2_connection_close(&conn)
    assert_eq_lit(close_result, based)
}

slay test_http2_performance() {
    test_start("HTTP/2 Performance Tests")
    
    fr fr Test multiple concurrent streams
    sus conn HTTP2Connection = http2_connection_create()
    sus concurrent_streams normie[10]
    
    fr fr Create 10 concurrent streams
    bestie i normie = 0; i < 10; i++ {
        sus headers tea[2]
        headers[0] = "accept: application/json"
        headers[1] = "x-stream-id: " + stringz.int_to_string(i)
        
        sus path tea = "/api/stream/" + stringz.int_to_string(i)
        concurrent_streams[i] = http2_send_request(&conn, "GET", path, headers, 2, "")
        assert_true(concurrent_streams[i] > 0)
    }
    
    fr fr Verify all streams are unique and odd (client-initiated)
    bestie i normie = 0; i < 10; i++ {
        assert_true(concurrent_streams[i] % 2 == 1)
        bestie j normie = i + 1; j < 10; j++ {
            assert_true(concurrent_streams[i] != concurrent_streams[j])
        }
    }
    
    fr fr Test large payload handling
    sus large_payload tea = ""
    bestie i normie = 0; i < 100; i++ {
        large_payload = stringz.concat(large_payload, "This is a test payload segment " + stringz.int_to_string(i) + ". ")
    }
    
    sus large_stream normie = http2_send_request(&conn, "POST", "/api/large", concurrent_streams, 0, large_payload)
    assert_true(large_stream > 0)
    
    http2_connection_close(&conn)
}

fr fr =============================================================================
fr fr TEST SUITE EXECUTION
fr fr =============================================================================

slay run_all_http2_tests() {
    vibez.spill("🧪 Running HTTP/2 Test Suite")
    vibez.spill("============================")
    
    fr fr Core frame handling tests
    test_http2_frame_creation()
    test_http2_frame_serialization()
    test_http2_frame_parsing()
    
    fr fr Connection and stream management tests
    test_http2_connection_management()
    test_http2_stream_management()
    
    fr fr Protocol feature tests
    test_http2_settings()
    test_hpack_compression()
    test_http2_request_response()
    test_http2_flow_control()
    test_http2_priority()
    test_http2_server_push()
    test_http2_error_handling()
    test_http2_ping()
    
    fr fr Interface tests
    test_http2_client()
    test_http2_server()
    
    fr fr Utility and integration tests
    test_http2_utility_functions()
    test_http2_connection_preface()
    test_http2_integration()
    test_http2_performance()
    
    print_test_summary()
    vibez.spill("✅ HTTP/2 Test Suite completed!")
}

fr fr Run the complete test suite
run_all_http2_tests()

fr fr Demo functions for manual testing
vibez.spill("")
vibez.spill("🚀 Running HTTP/2 Demos...")
http2_demo_client()
http2_demo_server()
http2_demo_features()
