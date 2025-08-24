// http2_comprehensive_test.csd - Comprehensive HTTP/2 Implementation Tests
// Complete test suite for HTTP/2 protocol features including frames, streams,
// multiplexing, flow control, server push, and HPACK compression

yeet "networkz/http2"
yeet "networkz/http2_advanced"
yeet "networkz/networkz"
yeet "testz"
yeet "stringz"
yeet "arrayz"
yeet "mathz"
yeet "timez"

// ==== FRAME PROCESSING TESTS ====

slay test_http2_frame_creation() {
    testz.test_start("HTTP/2 Frame Creation")
    
    // Test DATA frame creation
    sus data_frame Http2Frame = create_data_frame(1, "Hello, HTTP/2!", based) fam {
        when err -> {
            testz.test_fail("Failed to create DATA frame", err.message)
            damn
        }
    }
    
    testz.assert_eq_int(data_frame.frame_type, HTTP2_FRAME_DATA)
    testz.assert_eq_int(data_frame.stream_id, 1)
    testz.assert_eq_int(data_frame.flags, HTTP2_FLAG_END_STREAM)
    testz.assert_eq_str(data_frame.payload, "Hello, HTTP/2!")
    testz.assert_eq_int(data_frame.payload_length, 14)
    
    // Test HEADERS frame creation
    sus headers []tea = [
        ":method: GET",
        ":path: /test",
        ":scheme: https",
        ":authority: example.com",
        "user-agent: CURSED-HTTP2-Test/1.0"
    ]
    
    sus headers_frame Http2Frame = create_headers_frame(3, headers, no_cap, based) fam {
        when err -> {
            testz.test_fail("Failed to create HEADERS frame", err.message)
            damn
        }
    }
    
    testz.assert_eq_int(headers_frame.frame_type, HTTP2_FRAME_HEADERS)
    testz.assert_eq_int(headers_frame.stream_id, 3)
    testz.assert_eq_int(headers_frame.flags, HTTP2_FLAG_END_HEADERS)
    testz.assert_true(headers_frame.payload_length > 0)
    
    // Test SETTINGS frame creation
    sus settings Http2Settings = Http2Settings{
        header_table_size: 4096,
        enable_push: 1,
        max_concurrent_streams: 100,
        initial_window_size: 65535,
        max_frame_size: 16384,
        max_header_list_size: 8192
    }
    
    sus settings_frame Http2Frame = create_settings_frame(settings, no_cap)
    testz.assert_eq_int(settings_frame.frame_type, HTTP2_FRAME_SETTINGS)
    testz.assert_eq_int(settings_frame.stream_id, 0)
    testz.assert_eq_int(settings_frame.payload_length, 36)  // 6 settings * 6 bytes each
    
    // Test WINDOW_UPDATE frame creation
    sus window_frame Http2Frame = create_window_update_frame(5, 32768) fam {
        when err -> {
            testz.test_fail("Failed to create WINDOW_UPDATE frame", err.message)
            damn
        }
    }
    
    testz.assert_eq_int(window_frame.frame_type, HTTP2_FRAME_WINDOW_UPDATE)
    testz.assert_eq_int(window_frame.stream_id, 5)
    testz.assert_eq_int(window_frame.payload_length, 4)
    
    testz.test_pass("HTTP/2 frame creation successful")
}

slay test_http2_frame_parsing() {
    testz.test_start("HTTP/2 Frame Header Parsing")
    
    // Create a frame header manually
    sus header_bytes tea = create_frame_header(HTTP2_FRAME_DATA, HTTP2_FLAG_END_STREAM, 7, 100)
    testz.assert_eq_int(stringz.len(header_bytes), 9)
    
    // Parse the header back
    sus parsed_frame Http2Frame = parse_frame_header(header_bytes) fam {
        when err -> {
            testz.test_fail("Failed to parse frame header", err.message)
            damn
        }
    }
    
    testz.assert_eq_int(parsed_frame.frame_type, HTTP2_FRAME_DATA)
    testz.assert_eq_int(parsed_frame.flags, HTTP2_FLAG_END_STREAM)
    testz.assert_eq_int(parsed_frame.stream_id, 7)
    testz.assert_eq_int(parsed_frame.payload_length, 100)
    
    // Test invalid frame header (too short)
    sus invalid_header tea = "short"
    sus parse_result Http2Frame = parse_frame_header(invalid_header) fam {
        when err -> {
            testz.assert_true(stringz.contains(err.message, "Invalid frame header length"))
            testz.test_pass("Frame header parsing validation working")
            damn
        }
    }
    
    testz.test_fail("Should have failed to parse invalid frame header", "")
}

// ==== HPACK COMPRESSION TESTS ====

slay test_hpack_integer_encoding() {
    testz.test_start("HPACK Integer Encoding/Decoding")
    
    // Test small integer (fits in prefix)
    sus encoded tea = hpack_encode_integer(10, 5, 0x00)
    testz.assert_eq_int(stringz.len(encoded), 1)
    
    sus decoded_result [2]drip = hpack_decode_integer(encoded, 0, 5) fam {
        when err -> {
            testz.test_fail("Failed to decode small integer", err.message)
            damn
        }
    }
    testz.assert_eq_int(decoded_result[0], 10)
    testz.assert_eq_int(decoded_result[1], 1)
    
    // Test large integer (requires multiple bytes)
    sus large_encoded tea = hpack_encode_integer(1337, 5, 0x00)
    testz.assert_true(stringz.len(large_encoded) > 1)
    
    sus large_decoded_result [2]drip = hpack_decode_integer(large_encoded, 0, 5) fam {
        when err -> {
            testz.test_fail("Failed to decode large integer", err.message)
            damn
        }
    }
    testz.assert_eq_int(large_decoded_result[0], 1337)
    
    testz.test_pass("HPACK integer encoding/decoding successful")
}

slay test_hpack_string_encoding() {
    testz.test_start("HPACK String Encoding/Decoding")
    
    // Test literal string encoding
    sus test_string tea = "example.com"
    sus encoded_string tea = hpack_encode_string(test_string, no_cap)
    testz.assert_true(stringz.len(encoded_string) > stringz.len(test_string))
    
    // Test string decoding
    sus decoded_result [2]tea = hpack_decode_string(encoded_string, 0) fam {
        when err -> {
            testz.test_fail("Failed to decode string", err.message)
            damn
        }
    }
    testz.assert_eq_str(decoded_result[0], test_string)
    
    // Test empty string
    sus empty_encoded tea = hpack_encode_string("", no_cap)
    sus empty_decoded [2]tea = hpack_decode_string(empty_encoded, 0) fam {
        when err -> {
            testz.test_fail("Failed to decode empty string", err.message)
            damn
        }
    }
    testz.assert_eq_str(empty_decoded[0], "")
    
    testz.test_pass("HPACK string encoding/decoding successful")
}

slay test_hpack_dynamic_table() {
    testz.test_start("HPACK Dynamic Table Management")
    
    sus table HpackDynamicTable = create_hpack_dynamic_table(4096)
    testz.assert_eq_int(table.max_size, 4096)
    testz.assert_eq_int(table.size, 0)
    testz.assert_eq_int(table.insertion_count, 0)
    
    // Add entry to dynamic table
    sus entry HpackEntry = HpackEntry{
        name: "custom-header",
        value: "custom-value",
        size: 57  // name.length + value.length + 32
    }
    
    hpack_add_to_dynamic_table(table, entry) fam {
        when err -> {
            testz.test_fail("Failed to add entry to dynamic table", err.message)
            damn
        }
    }
    
    testz.assert_eq_int(table.insertion_count, 1)
    testz.assert_eq_int(table.size, 57)
    
    // Lookup entry from dynamic table
    sus retrieved_entry HpackEntry = hpack_lookup_dynamic(table, 1) fam {
        when err -> {
            testz.test_fail("Failed to lookup dynamic table entry", err.message)
            damn
        }
    }
    testz.assert_eq_str(retrieved_entry.name, "custom-header")
    testz.assert_eq_str(retrieved_entry.value, "custom-value")
    
    testz.test_pass("HPACK dynamic table management successful")
}

slay test_hpack_static_table() {
    testz.test_start("HPACK Static Table Lookup")
    
    // Test valid static table lookups
    sus authority_entry HpackEntry = hpack_lookup_static(1) fam {
        when err -> {
            testz.test_fail("Failed to lookup static table entry 1", err.message)
            damn
        }
    }
    testz.assert_eq_str(authority_entry.name, ":authority")
    
    sus method_get_entry HpackEntry = hpack_lookup_static(2) fam {
        when err -> {
            testz.test_fail("Failed to lookup static table entry 2", err.message)
            damn
        }
    }
    testz.assert_eq_str(method_get_entry.name, ":method")
    testz.assert_eq_str(method_get_entry.value, "GET")
    
    // Test invalid static table lookup
    sus invalid_entry HpackEntry = hpack_lookup_static(999) fam {
        when err -> {
            testz.assert_true(stringz.contains(err.message, "Invalid static table index"))
            testz.test_pass("Static table validation working")
            damn
        }
    }
    
    testz.test_fail("Should have failed to lookup invalid static table index", "")
}

// ==== STREAM MANAGEMENT TESTS ====

slay test_http2_stream_lifecycle() {
    testz.test_start("HTTP/2 Stream Lifecycle Management")
    
    // Create mock socket for testing
    sus mock_socket Socket = Socket{
        handle: 1234,
        socket_type: 1,
        family: 4,
        local_addr: "127.0.0.1",
        remote_addr: "example.com",
        local_port: 45678,
        remote_port: 443,
        state: 1,  // connected
        send_buffer_size: 8192,
        recv_buffer_size: 8192,
        timeout_seconds: 30,
        keep_alive: based,
        created_at: timez.now(),
        bytes_sent: 0,
        bytes_received: 0
    }
    
    // Create HTTP/2 connection
    sus conn Http2Connection = create_http2_connection(mock_socket, no_cap) fam {
        when err -> {
            testz.test_fail("Failed to create HTTP/2 connection", err.message)
            damn
        }
    }
    
    testz.assert_eq_int(conn.stream_count, 0)
    testz.assert_eq_int(conn.next_stream_id, 1)  // Client uses odd numbers
    testz.assert_false(conn.is_server)
    
    // Create a new stream
    sus stream Http2StreamState = create_stream(conn, 1) fam {
        when err -> {
            testz.test_fail("Failed to create stream", err.message)
            damn
        }
    }
    
    testz.assert_eq_int(stream.stream_id, 1)
    testz.assert_eq_int(stream.state, HTTP2_STREAM_IDLE)
    testz.assert_eq_int(stream.local_window, 65535)
    testz.assert_eq_int(stream.remote_window, 65535)
    testz.assert_eq_int(stream.priority_weight, 16)
    testz.assert_eq_int(conn.stream_count, 1)
    
    // Test stream state transitions
    transition_stream_state(stream, 1) fam {  // send/recv HEADERS
        when err -> {
            testz.test_fail("Failed to transition stream to OPEN", err.message)
            damn
        }
    }
    testz.assert_eq_int(stream.state, HTTP2_STREAM_OPEN)
    
    transition_stream_state(stream, 3) fam {  // send END_STREAM
        when err -> {
            testz.test_fail("Failed to transition stream to HALF_CLOSED_LOCAL", err.message)
            damn
        }
    }
    testz.assert_eq_int(stream.state, HTTP2_STREAM_HALF_CLOSED_LOCAL)
    
    transition_stream_state(stream, 4) fam {  // recv END_STREAM
        when err -> {
            testz.test_fail("Failed to transition stream to CLOSED", err.message)
            damn
        }
    }
    testz.assert_eq_int(stream.state, HTTP2_STREAM_CLOSED)
    
    testz.test_pass("HTTP/2 stream lifecycle successful")
}

// ==== FLOW CONTROL TESTS ====

slay test_http2_flow_control() {
    testz.test_start("HTTP/2 Flow Control Implementation")
    
    sus controller Http2FlowController = create_flow_controller(65535)
    testz.assert_eq_int(controller.connection_window, 65535)
    testz.assert_eq_int(controller.default_window_size, 65535)
    testz.assert_eq_int(controller.update_threshold, 32767)
    
    // Test flow control consumption
    flow_control_consume(controller, 1, 1000) fam {
        when err -> {
            testz.test_fail("Failed to consume from flow control window", err.message)
            damn
        }
    }
    
    testz.assert_eq_int(controller.connection_window, 64535)
    testz.assert_eq_int(controller.stream_windows[0], 64535)  // Stream ID 1 -> index 0
    testz.assert_eq_int(controller.pending_updates[0], 1000)
    
    // Test window update threshold
    testz.assert_false(flow_control_should_update(controller, 1))
    
    // Consume more to trigger update threshold
    flow_control_consume(controller, 1, 32000) fam {
        when err -> {
            testz.test_fail("Failed to consume large amount from flow control", err.message)
            damn
        }
    }
    
    testz.assert_true(flow_control_should_update(controller, 1))
    
    // Test exceeding window size
    sus exceed_result lit = flow_control_consume(controller, 1, 100000) fam {
        when err -> {
            testz.assert_true(stringz.contains(err.message, "exceeds"))
            testz.test_pass("Flow control window validation working")
            damn
        }
    }
    
    testz.test_fail("Should have failed when exceeding flow control window", "")
}

// ==== SERVER PUSH TESTS ====

slay test_http2_server_push() {
    testz.test_start("HTTP/2 Server Push Implementation")
    
    // Test PUSH_PROMISE frame creation
    sus push_headers []tea = [
        ":method: GET",
        ":path: /style.css",
        ":scheme: https",
        ":authority: example.com"
    ]
    
    sus push_promise_frame Http2Frame = create_push_promise_frame(1, 2, push_headers) fam {
        when err -> {
            testz.test_fail("Failed to create PUSH_PROMISE frame", err.message)
            damn
        }
    }
    
    testz.assert_eq_int(push_promise_frame.frame_type, HTTP2_FRAME_PUSH_PROMISE)
    testz.assert_eq_int(push_promise_frame.stream_id, 1)
    testz.assert_eq_int(push_promise_frame.flags, HTTP2_FLAG_END_HEADERS)
    testz.assert_true(push_promise_frame.payload_length > 4)  // At least 4 bytes for promised stream ID
    
    // Test invalid PUSH_PROMISE (same stream IDs)
    sus invalid_push Http2Frame = create_push_promise_frame(1, 1, push_headers) fam {
        when err -> {
            testz.assert_true(stringz.contains(err.message, "cannot equal"))
            testz.test_pass("PUSH_PROMISE validation working")
            damn
        }
    }
    
    testz.test_fail("Should have failed with same stream IDs", "")
}

// ==== PRIORITY TESTS ====

slay test_http2_priority_management() {
    testz.test_start("HTTP/2 Priority Frame Implementation")
    
    // Create priority frame
    sus priority_frame Http2Frame = create_priority_frame(5, 3, 200, based)
    testz.assert_eq_int(priority_frame.frame_type, HTTP2_FRAME_PRIORITY)
    testz.assert_eq_int(priority_frame.stream_id, 5)
    testz.assert_eq_int(priority_frame.payload_length, 5)
    testz.assert_eq_int(priority_frame.flags, 0)
    
    // Create mock connection for processing
    sus mock_socket Socket = Socket{
        handle: 5678,
        socket_type: 1,
        family: 4,
        local_addr: "127.0.0.1",
        remote_addr: "example.com",
        local_port: 45679,
        remote_port: 443,
        state: 1,
        send_buffer_size: 8192,
        recv_buffer_size: 8192,
        timeout_seconds: 30,
        keep_alive: based,
        created_at: timez.now(),
        bytes_sent: 0,
        bytes_received: 0
    }
    
    sus conn Http2Connection = create_http2_connection(mock_socket, no_cap) fam {
        when err -> {
            testz.test_fail("Failed to create connection for priority test", err.message)
            damn
        }
    }
    
    // Process priority frame
    sus stream_dep Http2StreamDependency = process_priority_frame(conn, priority_frame) fam {
        when err -> {
            testz.test_fail("Failed to process priority frame", err.message)
            damn
        }
    }
    
    testz.assert_eq_int(stream_dep.stream_id, 5)
    testz.assert_eq_int(stream_dep.depends_on, 3)
    testz.assert_eq_int(stream_dep.weight, 200)
    testz.assert_true(stream_dep.exclusive)
    
    testz.test_pass("HTTP/2 priority management successful")
}

// ==== MULTIPLEXED CONNECTION TESTS ====

slay test_http2_multiplexed_connection() {
    testz.test_start("HTTP/2 Multiplexed Connection Management")
    
    sus mock_socket Socket = Socket{
        handle: 9999,
        socket_type: 1,
        family: 4,
        local_addr: "127.0.0.1",
        remote_addr: "api.example.com",
        local_port: 45680,
        remote_port: 443,
        state: 1,
        send_buffer_size: 16384,
        recv_buffer_size: 16384,
        timeout_seconds: 60,
        keep_alive: based,
        created_at: timez.now(),
        bytes_sent: 0,
        bytes_received: 0
    }
    
    sus mux_conn Http2MultiplexedConnection = create_multiplexed_connection(mock_socket, no_cap) fam {
        when err -> {
            testz.test_fail("Failed to create multiplexed connection", err.message)
            damn
        }
    }
    
    testz.assert_eq_int(mux_conn.max_concurrent_streams, 1000)
    testz.assert_eq_int(mux_conn.stream_creation_rate, 0)
    testz.assert_false(mux_conn.connection.is_server)
    
    // Test stream creation
    sus stream_id drip = multiplex_create_stream(mux_conn) fam {
        when err -> {
            testz.test_fail("Failed to create multiplexed stream", err.message)
            damn
        }
    }
    
    testz.assert_eq_int(stream_id, 1)
    testz.assert_true(mux_conn.active_streams[0])  // Stream ID 1 -> index 0
    testz.assert_eq_int(mux_conn.connection.stream_count, 1)
    
    // Test stream closure
    multiplex_close_stream(mux_conn, stream_id) fam {
        when err -> {
            testz.test_fail("Failed to close multiplexed stream", err.message)
            damn
        }
    }
    
    testz.assert_false(mux_conn.active_streams[0])
    
    testz.test_pass("HTTP/2 multiplexed connection successful")
}

// ==== CONNECTION POOL TESTS ====

slay test_http2_connection_pool() {
    testz.test_start("HTTP/2 Connection Pool Management")
    
    sus pool Http2ConnectionPool = create_http2_connection_pool("api.example.com", 443, 10)
    testz.assert_eq_str(pool.host, "api.example.com")
    testz.assert_eq_int(pool.port, 443)
    testz.assert_eq_int(pool.max_connections, 10)
    testz.assert_eq_int(pool.active_connections, 0)
    testz.assert_eq_int(pool.connection_timeout, 30)
    testz.assert_eq_int(pool.idle_timeout, 300)
    testz.assert_eq_int(pool.health_check_interval, 60)
    
    testz.test_pass("HTTP/2 connection pool management successful")
}

// ==== INTEGRATION TESTS ====

slay test_http2_end_to_end_simulation() {
    testz.test_start("HTTP/2 End-to-End Simulation")
    
    // Simulate HTTP/2 GET request workflow
    sus mock_socket Socket = Socket{
        handle: 12345,
        socket_type: 1,
        family: 4,
        local_addr: "127.0.0.1",
        remote_addr: "httpbin.org",
        local_port: 45681,
        remote_port: 443,
        state: 1,
        send_buffer_size: 32768,
        recv_buffer_size: 32768,
        timeout_seconds: 30,
        keep_alive: based,
        created_at: timez.now(),
        bytes_sent: 0,
        bytes_received: 0
    }
    
    sus conn Http2Connection = create_http2_connection(mock_socket, no_cap) fam {
        when err -> {
            testz.test_fail("Failed to create connection for E2E test", err.message)
            damn
        }
    }
    
    // Simulate connection setup
    conn.connection_preface_sent = based
    conn.connection_preface_received = based
    conn.settings_sent = based
    conn.settings_acked = based
    
    // Create request stream
    sus stream Http2StreamState = create_stream(conn, 1) fam {
        when err -> {
            testz.test_fail("Failed to create request stream", err.message)
            damn
        }
    }
    
    // Create request headers
    sus request_headers []tea = [
        ":method: GET",
        ":path: /get",
        ":scheme: https",
        ":authority: httpbin.org",
        "user-agent: CURSED-HTTP2-Test/1.0",
        "accept: application/json"
    ]
    
    // Create HEADERS frame
    sus headers_frame Http2Frame = create_headers_frame(1, request_headers, based, based) fam {
        when err -> {
            testz.test_fail("Failed to create request headers frame", err.message)
            damn
        }
    }
    
    testz.assert_eq_int(headers_frame.frame_type, HTTP2_FRAME_HEADERS)
    testz.assert_eq_int(headers_frame.stream_id, 1)
    testz.assert_true(headers_frame.flags & HTTP2_FLAG_END_STREAM)
    testz.assert_true(headers_frame.flags & HTTP2_FLAG_END_HEADERS)
    
    // Simulate response headers
    sus response_headers []tea = [
        ":status: 200",
        "content-type: application/json",
        "content-length: 256",
        "server: nginx/1.20.1"
    ]
    
    sus response_headers_frame Http2Frame = create_headers_frame(1, response_headers, no_cap, based) fam {
        when err -> {
            testz.test_fail("Failed to create response headers frame", err.message)
            damn
        }
    }
    
    // Simulate response data
    sus response_body tea = "{\"args\": {}, \"headers\": {\"Accept\": \"application/json\", \"Host\": \"httpbin.org\", \"User-Agent\": \"CURSED-HTTP2-Test/1.0\"}, \"origin\": \"192.168.1.100\", \"url\": \"https://httpbin.org/get\"}"
    
    sus response_data_frame Http2Frame = create_data_frame(1, response_body, based) fam {
        when err -> {
            testz.test_fail("Failed to create response data frame", err.message)
            damn
        }
    }
    
    testz.assert_eq_int(response_data_frame.frame_type, HTTP2_FRAME_DATA)
    testz.assert_eq_int(response_data_frame.stream_id, 1)
    testz.assert_true(response_data_frame.flags & HTTP2_FLAG_END_STREAM)
    testz.assert_eq_int(response_data_frame.payload_length, stringz.len(response_body))
    
    testz.test_pass("HTTP/2 end-to-end simulation successful")
}

// ==== PERFORMANCE TESTS ====

slay test_http2_performance_characteristics() {
    testz.test_start("HTTP/2 Performance Characteristics")
    
    // Test frame creation performance
    sus start_time drip = timez.now()
    sus i drip = 0
    
    bestie (i < 1000) {
        sus test_data tea = stringz.concat(["Test data frame ", stringz.from_int(i)])
        sus data_frame Http2Frame = create_data_frame(i + 1, test_data, no_cap) fam {
            when err -> {
                testz.test_fail("Performance test failed creating frame", err.message)
                damn
            }
        }
        i = i + 1
    }
    
    sus end_time drip = timez.now()
    sus duration drip = end_time - start_time
    
    // Should be able to create 1000 frames in less than 1 second
    testz.assert_true(duration < 1000)
    
    // Test HPACK encoding performance
    sus hpack_start drip = timez.now()
    sus j drip = 0
    
    bestie (j < 500) {
        sus test_headers []tea = [
            stringz.concat(["custom-header-", stringz.from_int(j)]),
            stringz.concat(["custom-value-", stringz.from_int(j * 2)])
        ]
        
        sus encoded_headers tea = hpack_encode_string(test_headers[0], no_cap)
        encoded_headers = stringz.concat([encoded_headers, hpack_encode_string(test_headers[1], no_cap)])
        
        j = j + 1
    }
    
    sus hpack_end drip = timez.now()
    sus hpack_duration drip = hpack_end - hpack_start
    
    // HPACK encoding should be efficient
    testz.assert_true(hpack_duration < 2000)
    
    testz.test_pass("HTTP/2 performance characteristics acceptable")
}

// ==== ERROR HANDLING TESTS ====

slay test_http2_error_conditions() {
    testz.test_start("HTTP/2 Error Condition Handling")
    
    // Test invalid stream ID for DATA frame
    sus invalid_data Http2Frame = create_data_frame(0, "test", no_cap) fam {
        when err -> {
            testz.assert_true(stringz.contains(err.message, "stream ID 0"))
            testz.test_pass("DATA frame stream ID validation working")
            damn
        }
    }
    testz.test_fail("Should have failed with stream ID 0 for DATA frame", "")
}

// ==== MAIN TEST RUNNER ====

slay run_all_http2_tests() {
    testz.test_suite_start("HTTP/2 Comprehensive Test Suite")
    
    // Frame processing tests
    test_http2_frame_creation()
    test_http2_frame_parsing()
    
    // HPACK compression tests
    test_hpack_integer_encoding()
    test_hpack_string_encoding()
    test_hpack_dynamic_table()
    test_hpack_static_table()
    
    // Stream management tests
    test_http2_stream_lifecycle()
    
    // Flow control tests
    test_http2_flow_control()
    
    // Server push tests
    test_http2_server_push()
    
    // Priority tests
    test_http2_priority_management()
    
    // Multiplexed connection tests
    test_http2_multiplexed_connection()
    
    // Connection pool tests
    test_http2_connection_pool()
    
    // Integration tests
    test_http2_end_to_end_simulation()
    
    // Performance tests
    test_http2_performance_characteristics()
    
    // Error handling tests
    test_http2_error_conditions()
    
    testz.test_suite_end()
    testz.print_test_summary()
}

// Run all tests
run_all_http2_tests()
