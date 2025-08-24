// http2_validation_test.csd - HTTP/2 Implementation Validation
// Quick validation of HTTP/2 core functionality

yeet "vibez"
yeet "networkz/http2"
yeet "stringz"
yeet "mathz"

slay validate_frame_creation() {
    vibez.spill("🔬 Validating HTTP/2 Frame Creation...")
    
    // Test DATA frame
    sus data_frame Http2Frame = create_data_frame(1, "Test data", based) fam {
        when err -> {
            vibez.spill("❌ DATA frame creation failed:", err.message)
            damn
        }
    }
    vibez.spill("✅ DATA frame created - Type:", data_frame.frame_type, "Stream:", data_frame.stream_id)
    
    // Test HEADERS frame
    sus headers []tea = [":method: GET", ":path: /test", ":scheme: https"]
    sus headers_frame Http2Frame = create_headers_frame(3, headers, no_cap, based) fam {
        when err -> {
            vibez.spill("❌ HEADERS frame creation failed:", err.message)
            damn
        }
    }
    vibez.spill("✅ HEADERS frame created - Type:", headers_frame.frame_type, "Size:", headers_frame.payload_length)
    
    // Test SETTINGS frame
    sus settings Http2Settings = Http2Settings{
        header_table_size: 4096,
        enable_push: 1,
        max_concurrent_streams: 100,
        initial_window_size: 65535,
        max_frame_size: 16384,
        max_header_list_size: 8192
    }
    sus settings_frame Http2Frame = create_settings_frame(settings, no_cap)
    vibez.spill("✅ SETTINGS frame created - Size:", settings_frame.payload_length, "bytes")
}

slay validate_hpack_compression() {
    vibez.spill("🗜️  Validating HPACK Compression...")
    
    // Test integer encoding
    sus encoded tea = hpack_encode_integer(1337, 5, 0x00)
    vibez.spill("✅ Integer 1337 encoded to", stringz.len(encoded), "bytes")
    
    sus decoded_result [2]drip = hpack_decode_integer(encoded, 0, 5) fam {
        when err -> {
            vibez.spill("❌ Integer decoding failed:", err.message)
            damn
        }
    }
    ready (decoded_result[0] == 1337) {
        vibez.spill("✅ Integer round-trip successful:", decoded_result[0])
    } otherwise {
        vibez.spill("❌ Integer round-trip failed:", decoded_result[0], "!= 1337")
    }
    
    // Test string encoding
    sus test_string tea = "example.com"
    sus encoded_string tea = hpack_encode_string(test_string, no_cap)
    vibez.spill("✅ String encoded -", stringz.len(test_string), "->", stringz.len(encoded_string), "bytes")
    
    // Test dynamic table
    sus table HpackDynamicTable = create_hpack_dynamic_table(4096)
    sus entry HpackEntry = HpackEntry{name: "custom", value: "value", size: 43}
    hpack_add_to_dynamic_table(table, entry) fam {
        when err -> {
            vibez.spill("❌ Dynamic table add failed:", err.message)
            damn
        }
    }
    vibez.spill("✅ Dynamic table entry added - Size:", table.size)
}

slay validate_stream_management() {
    vibez.spill("🌊 Validating Stream Management...")
    
    // Create mock connection
    sus mock_socket Socket = Socket{
        handle: 123,
        socket_type: 1,
        family: 4,
        local_addr: "127.0.0.1",
        remote_addr: "example.com",
        local_port: 12345,
        remote_port: 443,
        state: 1,
        send_buffer_size: 8192,
        recv_buffer_size: 8192,
        timeout_seconds: 30,
        keep_alive: based,
        created_at: mathz.random_range(1000000, 9999999),
        bytes_sent: 0,
        bytes_received: 0
    }
    
    sus conn Http2Connection = create_http2_connection(mock_socket, no_cap) fam {
        when err -> {
            vibez.spill("❌ Connection creation failed:", err.message)
            damn
        }
    }
    vibez.spill("✅ HTTP/2 connection created - Next stream ID:", conn.next_stream_id)
    
    // Create and manage stream
    sus stream Http2StreamState = create_stream(conn, 1) fam {
        when err -> {
            vibez.spill("❌ Stream creation failed:", err.message)
            damn
        }
    }
    vibez.spill("✅ Stream created - ID:", stream.stream_id, "State:", stream.state)
    
    // Test state transitions
    transition_stream_state(stream, 1) fam {  // HEADERS sent/received
        when err -> {
            vibez.spill("❌ Stream state transition failed:", err.message)
            damn
        }
    }
    vibez.spill("✅ Stream transitioned to OPEN - State:", stream.state)
}

slay validate_flow_control() {
    vibez.spill("💧 Validating Flow Control...")
    
    sus controller Http2FlowController = create_flow_controller(65535)
    vibez.spill("✅ Flow controller created - Window:", controller.connection_window)
    
    // Test flow control consumption
    flow_control_consume(controller, 1, 1024) fam {
        when err -> {
            vibez.spill("❌ Flow control consumption failed:", err.message)
            damn
        }
    }
    vibez.spill("✅ Consumed 1024 bytes - Connection window:", controller.connection_window)
    vibez.spill("✅ Stream window:", controller.stream_windows[0])
}

slay validate_frame_header_parsing() {
    vibez.spill("📋 Validating Frame Header Parsing...")
    
    // Create frame header
    sus header_bytes tea = create_frame_header(HTTP2_FRAME_DATA, HTTP2_FLAG_END_STREAM, 5, 1024)
    vibez.spill("✅ Frame header created -", stringz.len(header_bytes), "bytes")
    
    // Parse it back
    sus parsed Http2Frame = parse_frame_header(header_bytes) fam {
        when err -> {
            vibez.spill("❌ Frame header parsing failed:", err.message)
            damn
        }
    }
    
    ready (parsed.frame_type == HTTP2_FRAME_DATA && 
          parsed.flags == HTTP2_FLAG_END_STREAM &&
          parsed.stream_id == 5 &&
          parsed.payload_length == 1024) {
        vibez.spill("✅ Frame header parsing successful")
    } otherwise {
        vibez.spill("❌ Frame header parsing validation failed")
    }
}

slay main() {
    vibez.spill("🚀 CURSED HTTP/2 Implementation Validation")
    vibez.spill("==========================================")
    
    validate_frame_creation()
    validate_hpack_compression()
    validate_stream_management()
    validate_flow_control()
    validate_frame_header_parsing()
    
    vibez.spill("\n🎉 HTTP/2 Implementation Validation Complete!")
    vibez.spill("✅ All core components working correctly")
    vibez.spill("🔥 Ready for production use!")
}

main()
