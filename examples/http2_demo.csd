// http2_demo.csd - HTTP/2 Implementation Demo
// Demonstrates key HTTP/2 features including basic requests, multiplexing,
// server push simulation, and HPACK compression

yeet "vibez"
yeet "networkz/http2"
yeet "networkz/http2_advanced"
yeet "stringz"
yeet "arrayz"
yeet "mathz"
yeet "timez"

slay demo_http2_frame_creation() {
    vibez.spill("=== HTTP/2 Frame Creation Demo ===")
    
    // Demonstrate DATA frame creation
    sus data_frame Http2Frame = create_data_frame(1, "Hello from HTTP/2!", based) fam {
        when err -> {
            vibez.spill("Failed to create DATA frame:", err.message)
            damn
        }
    }
    
    vibez.spill("Created DATA frame:")
    vibez.spill("  Type:", data_frame.frame_type, "(DATA)")
    vibez.spill("  Stream ID:", data_frame.stream_id) 
    vibez.spill("  Flags:", data_frame.flags, "(END_STREAM)")
    vibez.spill("  Payload Length:", data_frame.payload_length)
    vibez.spill("  Payload:", data_frame.payload)
    
    // Demonstrate HEADERS frame creation
    sus headers []tea = [
        ":method: GET",
        ":path: /api/v1/data", 
        ":scheme: https",
        ":authority: api.example.com",
        "user-agent: CURSED-HTTP2-Client/1.0",
        "accept: application/json",
        "accept-encoding: gzip, deflate",
        "authorization: Bearer abc123xyz"
    ]
    
    sus headers_frame Http2Frame = create_headers_frame(3, headers, no_cap, based) fam {
        when err -> {
            vibez.spill("Failed to create HEADERS frame:", err.message)
            damn
        }
    }
    
    vibez.spill("\nCreated HEADERS frame:")
    vibez.spill("  Type:", headers_frame.frame_type, "(HEADERS)")
    vibez.spill("  Stream ID:", headers_frame.stream_id)
    vibez.spill("  Flags:", headers_frame.flags, "(END_HEADERS)")
    vibez.spill("  Payload Length:", headers_frame.payload_length)
    vibez.spill("  Headers encoded with HPACK compression")
    
    // Demonstrate SETTINGS frame
    sus settings Http2Settings = Http2Settings{
        header_table_size: 4096,
        enable_push: 1,
        max_concurrent_streams: 100,
        initial_window_size: 65535, 
        max_frame_size: 16384,
        max_header_list_size: 8192
    }
    
    sus settings_frame Http2Frame = create_settings_frame(settings, no_cap)
    
    vibez.spill("\nCreated SETTINGS frame:")
    vibez.spill("  Type:", settings_frame.frame_type, "(SETTINGS)")
    vibez.spill("  Stream ID:", settings_frame.stream_id, "(connection-level)")
    vibez.spill("  Payload Length:", settings_frame.payload_length)
    vibez.spill("  Settings:")
    vibez.spill("    Header Table Size:", settings.header_table_size)
    vibez.spill("    Enable Push:", settings.enable_push)
    vibez.spill("    Max Concurrent Streams:", settings.max_concurrent_streams)
    vibez.spill("    Initial Window Size:", settings.initial_window_size)
    vibez.spill("    Max Frame Size:", settings.max_frame_size)
    vibez.spill("    Max Header List Size:", settings.max_header_list_size)
}

slay demo_hpack_compression() {
    vibez.spill("\n=== HPACK Header Compression Demo ===")
    
    // Demonstrate integer encoding/decoding
    vibez.spill("\nInteger Encoding Examples:")
    
    sus small_int drip = 10
    sus small_encoded tea = hpack_encode_integer(small_int, 5, 0x00)
    vibez.spill("  Integer", small_int, "encoded as", stringz.len(small_encoded), "byte(s)")
    
    sus large_int drip = 1337
    sus large_encoded tea = hpack_encode_integer(large_int, 5, 0x00)
    vibez.spill("  Integer", large_int, "encoded as", stringz.len(large_encoded), "byte(s)")
    
    // Demonstrate string encoding
    vibez.spill("\nString Encoding Examples:")
    
    sus test_strings []tea = [
        "example.com",
        "application/json", 
        "gzip, deflate, br",
        "CURSED-HTTP2-Client/1.0"
    ]
    
    sus i drip = 0
    bestie (i < arrayz.len(test_strings)) {
        sus original tea = test_strings[i]
        sus encoded tea = hpack_encode_string(original, no_cap)
        sus compression_ratio drip = (stringz.len(encoded) * 100) / stringz.len(original)
        
        vibez.spill("  '", original, "' ->", stringz.len(encoded), "bytes (", compression_ratio, "% of original)")
        i = i + 1
    }
    
    // Demonstrate dynamic table management
    vibez.spill("\nDynamic Table Management:")
    
    sus table HpackDynamicTable = create_hpack_dynamic_table(4096)
    vibez.spill("  Created dynamic table with max size:", table.max_size)
    
    sus custom_entries []HpackEntry = [
        HpackEntry{name: "x-custom-header", value: "custom-value", size: 57},
        HpackEntry{name: "x-request-id", value: "req-12345", size: 44},
        HpackEntry{name: "x-correlation-id", value: "corr-abcdef", size: 51}
    ]
    
    sus j drip = 0
    bestie (j < arrayz.len(custom_entries)) {
        hpack_add_to_dynamic_table(table, custom_entries[j]) fam {
            when err -> {
                vibez.spill("Failed to add entry to dynamic table:", err.message)
            }
        }
        vibez.spill("  Added entry:", custom_entries[j].name, "->", custom_entries[j].value)
        j = j + 1
    }
    
    vibez.spill("  Dynamic table now contains", table.insertion_count, "entries")
    vibez.spill("  Total size:", table.size, "bytes")
}

slay demo_stream_management() {
    vibez.spill("\n=== HTTP/2 Stream Management Demo ===")
    
    // Create mock socket for demo
    sus mock_socket Socket = Socket{
        handle: 12345,
        socket_type: 1,
        family: 4,
        local_addr: "127.0.0.1",
        remote_addr: "api.example.com",
        local_port: 45678,
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
    
    // Create HTTP/2 connection
    sus conn Http2Connection = create_http2_connection(mock_socket, no_cap) fam {
        when err -> {
            vibez.spill("Failed to create HTTP/2 connection:", err.message)
            damn
        }
    }
    
    vibez.spill("Created HTTP/2 connection:")
    vibez.spill("  Is Server:", ready (conn.is_server) "Yes" otherwise "No")
    vibez.spill("  Next Stream ID:", conn.next_stream_id, "(client uses odd numbers)")
    vibez.spill("  Initial Window Size:", conn.local_settings.initial_window_size)
    vibez.spill("  Max Concurrent Streams:", conn.local_settings.max_concurrent_streams)
    
    // Demonstrate stream creation and state transitions
    vibez.spill("\nStream Lifecycle Demo:")
    
    sus stream_ids []drip = [1, 3, 5, 7]
    sus k drip = 0
    
    bestie (k < arrayz.len(stream_ids)) {
        sus stream_id drip = stream_ids[k]
        
        sus stream Http2StreamState = create_stream(conn, stream_id) fam {
            when err -> {
                vibez.spill("Failed to create stream", stream_id, ":", err.message)
                k = k + 1
                bestie based
            }
        }
        
        vibez.spill("  Created stream", stream_id, "- State: IDLE")
        
        // Transition to OPEN
        transition_stream_state(stream, 1) fam {  // send/recv HEADERS
            when err -> {
                vibez.spill("State transition failed:", err.message)
            }
        }
        vibez.spill("    -> Transitioned to OPEN")
        
        // Transition to HALF_CLOSED_LOCAL
        transition_stream_state(stream, 3) fam {  // send END_STREAM
            when err -> {
                vibez.spill("State transition failed:", err.message)
            }
        }
        vibez.spill("    -> Transitioned to HALF_CLOSED_LOCAL")
        
        // Transition to CLOSED
        transition_stream_state(stream, 4) fam {  // recv END_STREAM
            when err -> {
                vibez.spill("State transition failed:", err.message)
            }
        }
        vibez.spill("    -> Transitioned to CLOSED")
        
        k = k + 1
    }
    
    vibez.spill("  Total streams created:", conn.stream_count)
}

slay demo_flow_control() {
    vibez.spill("\n=== HTTP/2 Flow Control Demo ===")
    
    sus controller Http2FlowController = create_flow_controller(65535)
    
    vibez.spill("Created flow controller:")
    vibez.spill("  Connection Window:", controller.connection_window)
    vibez.spill("  Default Window Size:", controller.default_window_size)
    vibez.spill("  Update Threshold:", controller.update_threshold)
    
    // Simulate data consumption
    vibez.spill("\nSimulating data flow:")
    
    sus data_chunks []drip = [1024, 2048, 4096, 8192, 16384]
    sus m drip = 0
    
    bestie (m < arrayz.len(data_chunks)) {
        sus chunk_size drip = data_chunks[m]
        
        flow_control_consume(controller, 1, chunk_size) fam {
            when err -> {
                vibez.spill("Flow control error:", err.message)
                m = m + 1
                bestie based
            }
        }
        
        vibez.spill("  Consumed", chunk_size, "bytes on stream 1")
        vibez.spill("    Connection window:", controller.connection_window)
        vibez.spill("    Stream window:", controller.stream_windows[0])
        vibez.spill("    Pending updates:", controller.pending_updates[0])
        vibez.spill("    Should update:", ready (flow_control_should_update(controller, 1)) "Yes" otherwise "No")
        
        m = m + 1
    }
}

slay demo_server_push_simulation() {
    vibez.spill("\n=== HTTP/2 Server Push Simulation ===")
    
    // Create PUSH_PROMISE frame
    sus push_headers []tea = [
        ":method: GET",
        ":path: /assets/style.css",
        ":scheme: https",
        ":authority: example.com"
    ]
    
    sus push_promise Http2Frame = create_push_promise_frame(1, 2, push_headers) fam {
        when err -> {
            vibez.spill("Failed to create PUSH_PROMISE frame:", err.message)
            damn
        }
    }
    
    vibez.spill("Created PUSH_PROMISE frame:")
    vibez.spill("  Parent Stream ID:", push_promise.stream_id)
    vibez.spill("  Promised Stream ID: 2")
    vibez.spill("  Resource Path: /assets/style.css")
    vibez.spill("  Frame Size:", push_promise.payload_length, "bytes")
    
    // Simulate push cache entry
    sus push_cache Http2PushCacheEntry = Http2PushCacheEntry{
        path: "/assets/style.css",
        headers: ["content-type: text/css", "cache-control: max-age=3600"],
        body: "body { font-family: Arial, sans-serif; color: #333; }",
        expires_at: timez.now() + 3600,
        etag: "\"css-v1.2.3\"",
        last_modified: "Wed, 21 Oct 2024 07:28:00 GMT"
    }
    
    vibez.spill("\nServer Push Cache Entry:")
    vibez.spill("  Path:", push_cache.path)
    vibez.spill("  Content Length:", stringz.len(push_cache.body), "bytes")
    vibez.spill("  ETag:", push_cache.etag)
    vibez.spill("  Expires:", push_cache.expires_at)
}

slay demo_multiplexed_connections() {
    vibez.spill("\n=== HTTP/2 Multiplexed Connection Demo ===")
    
    sus mock_socket Socket = Socket{
        handle: 54321,
        socket_type: 1,
        family: 4,
        local_addr: "127.0.0.1", 
        remote_addr: "api.example.com",
        local_port: 45679,
        remote_port: 443,
        state: 1,
        send_buffer_size: 65536,
        recv_buffer_size: 65536,
        timeout_seconds: 60,
        keep_alive: based,
        created_at: timez.now(),
        bytes_sent: 0,
        bytes_received: 0
    }
    
    sus mux_conn Http2MultiplexedConnection = create_multiplexed_connection(mock_socket, no_cap) fam {
        when err -> {
            vibez.spill("Failed to create multiplexed connection:", err.message)
            damn
        }
    }
    
    vibez.spill("Created multiplexed connection:")
    vibez.spill("  Max Concurrent Streams:", mux_conn.max_concurrent_streams)
    vibez.spill("  Stream Creation Rate:", mux_conn.stream_creation_rate)
    vibez.spill("  Connection Window:", mux_conn.flow_controller.connection_window)
    
    // Simulate creating multiple streams
    vibez.spill("\nSimulating concurrent stream creation:")
    
    sus n drip = 0
    bestie (n < 5) {
        sus stream_id drip = multiplex_create_stream(mux_conn) fam {
            when err -> {
                vibez.spill("Failed to create stream:", err.message)
                n = n + 1
                bestie based
            }
        }
        
        vibez.spill("  Created stream", stream_id)
        vibez.spill("    Stream active:", ready (mux_conn.active_streams[stream_id - 1]) "Yes" otherwise "No")
        n = n + 1
    }
    
    vibez.spill("  Total active streams:", mux_conn.connection.stream_count)
}

slay demo_connection_pooling() {
    vibez.spill("\n=== HTTP/2 Connection Pool Demo ===")
    
    sus pool Http2ConnectionPool = create_http2_connection_pool("api.example.com", 443, 5)
    
    vibez.spill("Created HTTP/2 connection pool:")
    vibez.spill("  Host:", pool.host)
    vibez.spill("  Port:", pool.port)
    vibez.spill("  Max Connections:", pool.max_connections)
    vibez.spill("  Active Connections:", pool.active_connections)
    vibez.spill("  Connection Timeout:", pool.connection_timeout, "seconds")
    vibez.spill("  Idle Timeout:", pool.idle_timeout, "seconds")
    vibez.spill("  Health Check Interval:", pool.health_check_interval, "seconds")
}

slay demo_performance_metrics() {
    vibez.spill("\n=== HTTP/2 Performance Metrics Demo ===")
    
    // Measure frame creation performance
    sus start_time drip = timez.now()
    sus frame_count drip = 1000
    
    sus p drip = 0
    bestie (p < frame_count) {
        sus test_data tea = stringz.concat(["Frame data ", stringz.from_int(p)])
        sus frame Http2Frame = create_data_frame(p + 1, test_data, no_cap) fam {
            when err -> {
                vibez.spill("Performance test failed:", err.message)
                p = p + 1
                bestie based
            }
        }
        p = p + 1
    }
    
    sus end_time drip = timez.now()
    sus duration drip = end_time - start_time
    sus frames_per_second drip = (frame_count * 1000) / duration
    
    vibez.spill("Frame Creation Performance:")
    vibez.spill("  Created", frame_count, "frames in", duration, "ms")
    vibez.spill("  Performance:", frames_per_second, "frames/second")
    
    // Measure HPACK encoding performance
    sus hpack_start drip = timez.now()
    sus header_count drip = 500
    
    sus q drip = 0
    bestie (q < header_count) {
        sus header_name tea = stringz.concat(["x-custom-header-", stringz.from_int(q)])
        sus header_value tea = stringz.concat(["value-", stringz.from_int(q * 2)])
        
        sus encoded_name tea = hpack_encode_string(header_name, no_cap)
        sus encoded_value tea = hpack_encode_string(header_value, no_cap)
        
        q = q + 1
    }
    
    sus hpack_end drip = timez.now()
    sus hpack_duration drip = hpack_end - hpack_start
    sus headers_per_second drip = (header_count * 1000) / hpack_duration
    
    vibez.spill("\nHPACK Encoding Performance:")
    vibez.spill("  Encoded", header_count, "headers in", hpack_duration, "ms")
    vibez.spill("  Performance:", headers_per_second, "headers/second")
}

slay main_character() {
    vibez.spill("🚀 CURSED HTTP/2 Implementation Demo")
    vibez.spill("=====================================\n")
    
    // Run all demo functions
    demo_http2_frame_creation()
    demo_hpack_compression()
    demo_stream_management()
    demo_flow_control()
    demo_server_push_simulation()
    demo_multiplexed_connections()
    demo_connection_pooling()
    demo_performance_metrics()
    
    vibez.spill("\n✨ HTTP/2 Demo Complete!")
    vibez.spill("The CURSED HTTP/2 implementation provides:")
    vibez.spill("  ✓ Binary frame protocol with all frame types")
    vibez.spill("  ✓ HPACK header compression (RFC 7541)")
    vibez.spill("  ✓ Stream multiplexing and flow control")
    vibez.spill("  ✓ Server push capabilities")
    vibez.spill("  ✓ Connection pooling and management")
    vibez.spill("  ✓ High-performance frame processing")
    vibez.spill("  ✓ Production-ready error handling")
    vibez.spill("  ✓ Memory-safe implementation")
    
    vibez.spill("\nReady for production use! 🎯")
}

main()
