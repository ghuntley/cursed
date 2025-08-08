yeet "testz"
yeet "websocketz"
yeet "stringz"

fr fr ========================================
fr fr CURSED WebSocket Module Test Suite
fr fr Comprehensive Testing for websocketz
fr fr ========================================

slay test_websocket_frame_creation() {
    test_start("WebSocket Frame Creation")
    
    fr fr Test TEXT frame creation
    sus text_frame WebSocketFrame = ws_frame_create(WS_OPCODE_TEXT, "Hello WebSocket!", based)
    assert_eq_int(text_frame.opcode, WS_OPCODE_TEXT)
    assert_eq_lit(text_frame.fin, based)
    assert_eq_lit(text_frame.masked, based)
    assert_eq_string(text_frame.payload, "Hello WebSocket!")
    assert_eq_int(text_frame.payload_length, stringz.length("Hello WebSocket!"))
    
    fr fr Test BINARY frame creation
    sus binary_frame WebSocketFrame = ws_frame_create(WS_OPCODE_BINARY, "binary_data", cap)
    assert_eq_int(binary_frame.opcode, WS_OPCODE_BINARY)
    assert_eq_lit(binary_frame.fin, based)
    assert_eq_lit(binary_frame.masked, cap)
    assert_eq_string(binary_frame.payload, "binary_data")
    
    fr fr Test PING frame creation
    sus ping_frame WebSocketFrame = ws_frame_create(WS_OPCODE_PING, "ping_test", based)
    assert_eq_int(ping_frame.opcode, WS_OPCODE_PING)
    assert_eq_string(ping_frame.payload, "ping_test")
    
    fr fr Test CLOSE frame creation
    sus close_frame WebSocketFrame = ws_frame_create(WS_OPCODE_CLOSE, "1000:Normal closure", cap)
    assert_eq_int(close_frame.opcode, WS_OPCODE_CLOSE)
    assert_true(stringz.contains(close_frame.payload, "1000"))
    assert_true(stringz.contains(close_frame.payload, "Normal closure"))
}

slay test_websocket_frame_serialization() {
    test_start("WebSocket Frame Serialization")
    
    fr fr Create test frame
    sus frame WebSocketFrame = ws_frame_create(WS_OPCODE_TEXT, "test payload", based)
    
    fr fr Serialize frame
    sus serialized tea = ws_frame_serialize(frame)
    
    fr fr Verify serialization format
    assert_true(stringz.starts_with(serialized, "WS-FRAME:"))
    assert_true(stringz.contains(serialized, "test payload"))
    assert_true(stringz.contains(serialized, stringz.int_to_string(WS_OPCODE_TEXT)))
    
    fr fr Test different frame types
    sus binary_frame WebSocketFrame = ws_frame_create(WS_OPCODE_BINARY, "binary", cap)
    sus binary_serialized tea = ws_frame_serialize(binary_frame)
    assert_true(stringz.starts_with(binary_serialized, "WS-FRAME:"))
    assert_true(stringz.contains(binary_serialized, "binary"))
}

slay test_websocket_frame_parsing() {
    test_start("WebSocket Frame Parsing")
    
    fr fr Test valid frame parsing
    sus valid_data tea = "WS-FRAME:129:20:0:12:34:56:78:Hello WebSocket"
    sus parsed_frame WebSocketFrame = ws_frame_parse(valid_data)
    assert_eq_int(parsed_frame.opcode, WS_OPCODE_TEXT)
    assert_eq_lit(parsed_frame.fin, based)
    assert_eq_string(parsed_frame.payload, "simulated-ws-payload")
    
    fr fr Test invalid frame parsing
    sus invalid_data tea = "INVALID-FRAME-DATA"
    sus invalid_frame WebSocketFrame = ws_frame_parse(invalid_data)
    assert_eq_int(invalid_frame.opcode, 255)  fr fr Invalid opcode
    assert_eq_int(invalid_frame.payload_length, 0)
    assert_eq_string(invalid_frame.payload, "")
}

slay test_websocket_handshake() {
    test_start("WebSocket Handshake")
    
    fr fr Test key generation
    sus key tea = ws_generate_key()
    assert_true(stringz.length(key) > 0)
    assert_true(ws_validate_key(key))
    
    fr fr Test key validation
    assert_true(ws_validate_key("dGhlIHNhbXBsZSBub25jZQ=="))  fr fr Valid base64 key
    assert_false(ws_validate_key(""))  fr fr Empty key
    assert_false(ws_validate_key("short"))  fr fr Too short
    
    fr fr Test accept key computation
    sus client_key tea = "dGhlIHNhbXBsZSBub25jZQ=="
    sus accept_key tea = ws_compute_accept_key(client_key)
    assert_true(stringz.length(accept_key) > 0)
    
    fr fr Test handshake request creation
    sus protocols [3]tea
    protocols[0] = "chat"
    protocols[1] = "echo"
    sus request tea = ws_create_handshake_request("/websocket", protocols, 2)
    
    assert_true(stringz.contains(request, "GET /websocket HTTP/1.1"))
    assert_true(stringz.contains(request, "Upgrade: websocket"))
    assert_true(stringz.contains(request, "Connection: Upgrade"))
    assert_true(stringz.contains(request, "Sec-WebSocket-Key:"))
    assert_true(stringz.contains(request, "Sec-WebSocket-Version: 13"))
    assert_true(stringz.contains(request, "Sec-WebSocket-Protocol: chat, echo"))
    
    fr fr Test handshake response creation
    sus response tea = ws_create_handshake_response(client_key, "chat")
    assert_true(stringz.contains(response, "HTTP/1.1 101 Switching Protocols"))
    assert_true(stringz.contains(response, "Upgrade: websocket"))
    assert_true(stringz.contains(response, "Connection: Upgrade"))
    assert_true(stringz.contains(response, "Sec-WebSocket-Accept:"))
    assert_true(stringz.contains(response, "Sec-WebSocket-Protocol: chat"))
}

slay test_websocket_handshake_validation() {
    test_start("WebSocket Handshake Validation")
    
    fr fr Test valid handshake request
    sus valid_request tea = "GET /ws HTTP/1.1\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\n\r\n"
    assert_true(ws_validate_handshake_request(valid_request))
    
    fr fr Test invalid handshake requests
    sus no_upgrade tea = "GET /ws HTTP/1.1\r\nConnection: Upgrade\r\n\r\n"
    assert_false(ws_validate_handshake_request(no_upgrade))
    
    sus no_connection tea = "GET /ws HTTP/1.1\r\nUpgrade: websocket\r\n\r\n"
    assert_false(ws_validate_handshake_request(no_connection))
    
    sus no_key tea = "GET /ws HTTP/1.1\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n\r\n"
    assert_false(ws_validate_handshake_request(no_key))
    
    sus no_version tea = "GET /ws HTTP/1.1\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: test\r\n\r\n"
    assert_false(ws_validate_handshake_request(no_version))
    
    fr fr Test valid handshake response
    sus expected_accept tea = ws_compute_accept_key("dGhlIHNhbXBsZSBub25jZQ==")
    sus valid_response tea = "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: " + expected_accept + "\r\n\r\n"
    assert_true(ws_validate_handshake_response(valid_response, expected_accept))
    
    fr fr Test invalid handshake response
    sus wrong_accept tea = "wrongacceptkey"
    assert_false(ws_validate_handshake_response(valid_response, wrong_accept))
}

slay test_websocket_connection_management() {
    test_start("WebSocket Connection Management")
    
    fr fr Test connection creation
    sus conn WebSocketConnection = ws_connection_create("ws://localhost:8080/test", cap)
    assert_eq_int(conn.connection_id, 1)
    assert_eq_int(conn.state, WS_STATE_CONNECTING)
    assert_eq_string(conn.url, "ws://localhost:8080/test")
    assert_eq_string(conn.protocol, "")
    assert_eq_lit(conn.is_server, cap)
    assert_eq_int(conn.max_frame_size, 1048576)
    assert_eq_int(conn.ping_interval, 30)
    assert_eq_int(conn.queue_size, 0)
    
    fr fr Test connection opening
    sus open_result lit = ws_connection_open(&conn)
    assert_eq_lit(open_result, based)
    assert_eq_int(conn.state, WS_STATE_OPEN)
    assert_true(ws_connection_is_open(conn))
    
    fr fr Test protocol setting
    sus protocol_result lit = ws_connection_set_protocol(&conn, "chat")
    assert_eq_lit(protocol_result, based)
    assert_eq_string(conn.protocol, "chat")
    
    fr fr Test connection closing
    sus close_result lit = ws_connection_close(&conn, WS_CLOSE_NORMAL, "Test closing")
    assert_eq_lit(close_result, based)
    assert_eq_int(conn.state, WS_STATE_CLOSING)
}

slay test_websocket_messaging() {
    test_start("WebSocket Messaging")
    
    fr fr Create open connection
    sus conn WebSocketConnection = ws_connection_create("ws://localhost:8080/test", cap)
    ws_connection_open(&conn)
    
    fr fr Test sending text message
    sus text_result lit = ws_send_text(&conn, "Hello WebSocket!")
    assert_eq_lit(text_result, based)
    
    fr fr Test sending binary message
    sus binary_result lit = ws_send_binary(&conn, "binary_test_data")
    assert_eq_lit(binary_result, based)
    
    fr fr Test sending ping
    sus ping_result lit = ws_send_ping(&conn, "ping_payload")
    assert_eq_lit(ping_result, based)
    assert_eq_int(conn.last_ping_time, 1234567890)
    
    fr fr Test sending pong
    sus pong_result lit = ws_send_pong(&conn, "pong_payload")
    assert_eq_lit(pong_result, based)
    
    fr fr Test message too large
    sus large_message tea = ""
    bestie i normie = 0; i < 100000; i++ {  fr fr Create message larger than max_frame_size
        large_message = stringz.concat(large_message, "x")
    }
    sus large_result lit = ws_send_text(&conn, large_message)
    assert_eq_lit(large_result, cap)  fr fr Should fail due to size limit
    
    fr fr Test messaging on closed connection
    ws_connection_close(&conn, WS_CLOSE_NORMAL, "Test")
    sus closed_result lit = ws_send_text(&conn, "Should fail")
    assert_eq_lit(closed_result, cap)  fr fr Should fail on closed connection
}

slay test_websocket_message_queue() {
    test_start("WebSocket Message Queue")
    
    fr fr Create connection
    sus conn WebSocketConnection = ws_connection_create("ws://localhost:8080/test", cap)
    ws_connection_open(&conn)
    
    fr fr Test queuing messages
    sus queue_result1 lit = ws_queue_message(&conn, WS_OPCODE_TEXT, "Message 1")
    assert_eq_lit(queue_result1, based)
    assert_eq_int(conn.queue_size, 1)
    
    sus queue_result2 lit = ws_queue_message(&conn, WS_OPCODE_TEXT, "Message 2")
    assert_eq_lit(queue_result2, based)
    assert_eq_int(conn.queue_size, 2)
    
    sus queue_result3 lit = ws_queue_message(&conn, WS_OPCODE_BINARY, "Binary data")
    assert_eq_lit(queue_result3, based)
    assert_eq_int(conn.queue_size, 3)
    
    fr fr Test receiving messages (FIFO order)
    sus msg1 WebSocketMessage = ws_receive_message(&conn)
    assert_eq_int(msg1.message_type, WS_OPCODE_TEXT)
    assert_eq_string(msg1.payload, "Message 1")
    assert_eq_int(msg1.connection_id, conn.connection_id)
    assert_eq_int(conn.queue_size, 2)
    
    sus msg2 WebSocketMessage = ws_receive_message(&conn)
    assert_eq_string(msg2.payload, "Message 2")
    assert_eq_int(conn.queue_size, 1)
    
    sus msg3 WebSocketMessage = ws_receive_message(&conn)
    assert_eq_int(msg3.message_type, WS_OPCODE_TEXT)  fr fr Simulated as text
    assert_eq_string(msg3.payload, "Binary data")
    assert_eq_int(conn.queue_size, 0)
    
    fr fr Test receiving from empty queue
    sus empty_msg WebSocketMessage = ws_receive_message(&conn)
    assert_eq_int(empty_msg.message_type, 0)
    assert_eq_string(empty_msg.payload, "")
    assert_eq_int(empty_msg.connection_id, 0)
}

slay test_websocket_room_management() {
    test_start("WebSocket Room Management")
    
    fr fr Test room creation
    sus room WebSocketRoom = ws_room_create("room123", "Test Chat Room")
    assert_eq_string(room.room_id, "room123")
    assert_eq_string(room.name, "Test Chat Room")
    assert_eq_int(room.connection_count, 0)
    assert_eq_int(room.max_connections, 50)
    assert_eq_int(room.created_time, 1234567890)
    
    fr fr Test joining room
    sus join_result1 lit = ws_room_join(&room, 1001)
    assert_eq_lit(join_result1, based)
    assert_eq_int(room.connection_count, 1)
    assert_eq_int(room.connections[0], 1001)
    
    sus join_result2 lit = ws_room_join(&room, 1002)
    assert_eq_lit(join_result2, based)
    assert_eq_int(room.connection_count, 2)
    assert_eq_int(room.connections[1], 1002)
    
    fr fr Test duplicate join (should succeed but not duplicate)
    sus duplicate_join lit = ws_room_join(&room, 1001)
    assert_eq_lit(duplicate_join, based)
    assert_eq_int(room.connection_count, 2)  fr fr Count should remain the same
    
    fr fr Test room utilities
    assert_eq_int(ws_room_get_connection_count(room), 2)
    assert_false(ws_room_is_empty(room))
    
    fr fr Test broadcasting
    sus broadcast_count normie = ws_room_broadcast(room, "Test broadcast message")
    assert_eq_int(broadcast_count, 2)  fr fr Should broadcast to 2 connections
    
    fr fr Test leaving room
    sus leave_result1 lit = ws_room_leave(&room, 1001)
    assert_eq_lit(leave_result1, based)
    assert_eq_int(room.connection_count, 1)
    assert_eq_int(room.connections[0], 1002)  fr fr 1002 should be shifted to position 0
    
    sus leave_result2 lit = ws_room_leave(&room, 1002)
    assert_eq_lit(leave_result2, based)
    assert_eq_int(room.connection_count, 0)
    assert_true(ws_room_is_empty(room))
    
    fr fr Test leaving non-existent connection
    sus nonexistent_leave lit = ws_room_leave(&room, 9999)
    assert_eq_lit(nonexistent_leave, cap)  fr fr Should fail
}

slay test_websocket_client() {
    test_start("WebSocket Client")
    
    fr fr Test client connection
    sus protocols [3]tea
    protocols[0] = "chat"
    protocols[1] = "echo"
    
    sus client WebSocketConnection = ws_client_connect("ws://localhost:8080/websocket", protocols, 2)
    assert_eq_string(client.url, "ws://localhost:8080/websocket")
    assert_eq_lit(client.is_server, cap)
    
    fr fr Should be connected if handshake succeeds
    assert_eq_int(client.state, WS_STATE_OPEN)
    assert_eq_string(client.protocol, "chat")  fr fr Should select first protocol
    
    fr fr Test client disconnect
    sus disconnect_result lit = ws_client_disconnect(&client)
    assert_eq_lit(disconnect_result, based)
    assert_eq_int(client.state, WS_STATE_CLOSING)
    
    fr fr Test connection with no protocols
    sus simple_client WebSocketConnection = ws_client_connect("ws://localhost:8080/simple", protocols, 0)
    assert_eq_string(simple_client.protocol, "")  fr fr No protocol selected
}

slay test_websocket_server() {
    test_start("WebSocket Server")
    
    fr fr Test server creation
    sus server WebSocketConnection = ws_server_create(8080, "/websocket")
    assert_true(stringz.contains(server.url, "8080"))
    assert_true(stringz.contains(server.url, "/websocket"))
    assert_eq_lit(server.is_server, based)
    assert_eq_int(server.state, WS_STATE_OPEN)
    
    fr fr Test upgrade request handling
    sus valid_request tea = "GET /websocket HTTP/1.1\r\nHost: localhost:8080\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\n\r\n"
    sus upgrade_response tea = ws_server_handle_upgrade(valid_request)
    assert_true(stringz.contains(upgrade_response, "101 Switching Protocols"))
    assert_true(stringz.contains(upgrade_response, "Sec-WebSocket-Accept:"))
    
    fr fr Test invalid upgrade request
    sus invalid_request tea = "GET / HTTP/1.1\r\nHost: localhost:8080\r\n\r\n"
    sus invalid_response tea = ws_server_handle_upgrade(invalid_request)
    assert_true(stringz.contains(invalid_response, "400 Bad Request"))
    
    fr fr Test client acceptance
    sus client_conn WebSocketConnection = ws_server_accept_connection(&server, valid_request)
    assert_eq_lit(client_conn.is_server, cap)
    assert_eq_int(client_conn.state, WS_STATE_OPEN)
}

slay test_websocket_utility_functions() {
    test_start("WebSocket Utility Functions")
    
    fr fr Test opcode names
    assert_eq_string(ws_get_opcode_name(WS_OPCODE_CONTINUATION), "CONTINUATION")
    assert_eq_string(ws_get_opcode_name(WS_OPCODE_TEXT), "TEXT")
    assert_eq_string(ws_get_opcode_name(WS_OPCODE_BINARY), "BINARY")
    assert_eq_string(ws_get_opcode_name(WS_OPCODE_CLOSE), "CLOSE")
    assert_eq_string(ws_get_opcode_name(WS_OPCODE_PING), "PING")
    assert_eq_string(ws_get_opcode_name(WS_OPCODE_PONG), "PONG")
    assert_eq_string(ws_get_opcode_name(255), "UNKNOWN")
    
    fr fr Test close code names
    assert_eq_string(ws_get_close_code_name(WS_CLOSE_NORMAL), "NORMAL_CLOSURE")
    assert_eq_string(ws_get_close_code_name(WS_CLOSE_GOING_AWAY), "GOING_AWAY")
    assert_eq_string(ws_get_close_code_name(WS_CLOSE_PROTOCOL_ERROR), "PROTOCOL_ERROR")
    assert_eq_string(ws_get_close_code_name(WS_CLOSE_INTERNAL_ERROR), "INTERNAL_SERVER_ERROR")
    assert_eq_string(ws_get_close_code_name(9999), "UNKNOWN_CODE")
    
    fr fr Test state names
    assert_eq_string(ws_get_state_name(WS_STATE_CONNECTING), "CONNECTING")
    assert_eq_string(ws_get_state_name(WS_STATE_OPEN), "OPEN")
    assert_eq_string(ws_get_state_name(WS_STATE_CLOSING), "CLOSING")
    assert_eq_string(ws_get_state_name(WS_STATE_CLOSED), "CLOSED")
    assert_eq_string(ws_get_state_name(255), "UNKNOWN_STATE")
    
    fr fr Test frame type classification
    assert_true(ws_is_control_frame(WS_OPCODE_CLOSE))
    assert_true(ws_is_control_frame(WS_OPCODE_PING))
    assert_true(ws_is_control_frame(WS_OPCODE_PONG))
    assert_false(ws_is_control_frame(WS_OPCODE_TEXT))
    assert_false(ws_is_control_frame(WS_OPCODE_BINARY))
    
    assert_true(ws_is_data_frame(WS_OPCODE_TEXT))
    assert_true(ws_is_data_frame(WS_OPCODE_BINARY))
    assert_true(ws_is_data_frame(WS_OPCODE_CONTINUATION))
    assert_false(ws_is_data_frame(WS_OPCODE_CLOSE))
    assert_false(ws_is_data_frame(WS_OPCODE_PING))
    
    fr fr Test UTF-8 validation
    assert_true(ws_validate_utf8("Valid UTF-8 text"))
    assert_true(ws_validate_utf8("Text with émojis 🚀"))
    assert_false(ws_validate_utf8("Text with null\0byte"))
}

slay test_websocket_extensions() {
    test_start("WebSocket Extensions")
    
    fr fr Test extension parsing
    sus deflate_ext tea = ws_extension_parse("permessage-deflate; client_max_window_bits")
    assert_eq_string(deflate_ext, "permessage-deflate")
    
    sus webkit_ext tea = ws_extension_parse("x-webkit-deflate-frame")
    assert_eq_string(webkit_ext, "x-webkit-deflate-frame")
    
    sus unknown_ext tea = ws_extension_parse("unknown-extension")
    assert_eq_string(unknown_ext, "")
    
    fr fr Test extension negotiation
    sus client_exts tea = "permessage-deflate; client_max_window_bits, x-webkit-deflate-frame"
    sus server_exts tea = "permessage-deflate"
    sus negotiated tea = ws_extension_negotiate(client_exts, server_exts)
    assert_eq_string(negotiated, "permessage-deflate")
    
    sus no_common tea = ws_extension_negotiate("unknown-ext", "other-ext")
    assert_eq_string(no_common, "")
    
    fr fr Test compression/decompression
    sus original tea = "This is test data for compression"
    sus compressed tea = ws_extension_compress(original, "permessage-deflate")
    assert_true(stringz.starts_with(compressed, "COMPRESSED:"))
    
    sus decompressed tea = ws_extension_decompress(compressed, "permessage-deflate")
    assert_eq_string(decompressed, original)
    
    fr fr Test no compression
    sus uncompressed tea = ws_extension_compress(original, "")
    assert_eq_string(uncompressed, original)
}

slay test_websocket_security() {
    test_start("WebSocket Security")
    
    fr fr Test origin validation
    sus allowed_origins [3]tea
    allowed_origins[0] = "https://example.com"
    allowed_origins[1] = "https://app.example.com"
    allowed_origins[2] = "*"
    
    assert_true(ws_validate_origin("https://example.com", allowed_origins, 2))
    assert_true(ws_validate_origin("https://app.example.com", allowed_origins, 2))
    assert_false(ws_validate_origin("https://malicious.com", allowed_origins, 2))
    assert_false(ws_validate_origin("", allowed_origins, 2))
    
    fr fr Test wildcard origin
    assert_true(ws_validate_origin("https://any-domain.com", allowed_origins, 3))
    
    fr fr Test rate limiting
    assert_true(ws_rate_limit_check(1001, 60))  fr fr Should allow (simulated)
    assert_true(ws_rate_limit_check(1002, 30))  fr fr Should allow (simulated)
    
    fr fr Test content filtering
    sus blocked_words [3]tea
    blocked_words[0] = "spam"
    blocked_words[1] = "malicious"
    blocked_words[2] = "forbidden"
    
    assert_true(ws_content_filter("This is a clean message", blocked_words, 3))
    assert_false(ws_content_filter("This message contains spam", blocked_words, 3))
    assert_false(ws_content_filter("Malicious content here", blocked_words, 3))
    assert_false(ws_content_filter("This is forbidden content", blocked_words, 3))
}

slay test_websocket_integration() {
    test_start("WebSocket Integration Test")
    
    fr fr Create server and client
    sus server WebSocketConnection = ws_server_create(8080, "/chat")
    sus protocols [2]tea
    protocols[0] = "chat"
    protocols[1] = "echo"
    sus client WebSocketConnection = ws_client_connect("ws://localhost:8080/chat", protocols, 2)
    
    fr fr Both should be open
    assert_true(ws_connection_is_open(server))
    assert_true(ws_connection_is_open(client))
    
    fr fr Create chat room and add client
    sus room WebSocketRoom = ws_room_create("integration", "Integration Test Room")
    ws_room_join(&room, client.connection_id)
    
    fr fr Test message flow
    ws_send_text(&client, "Hello from client!")
    ws_queue_message(&client, WS_OPCODE_TEXT, "Server response")
    
    sus received_msg WebSocketMessage = ws_receive_message(&client)
    assert_eq_string(received_msg.payload, "Server response")
    
    fr fr Test broadcasting
    sus broadcast_count normie = ws_room_broadcast(room, "Broadcast message")
    assert_eq_int(broadcast_count, 1)
    
    fr fr Test ping/pong exchange
    ws_send_ping(&client, "ping_test")
    ws_send_pong(&server, "pong_response")
    
    fr fr Test graceful disconnect
    ws_connection_close(&client, WS_CLOSE_NORMAL, "Integration test complete")
    assert_eq_int(client.state, WS_STATE_CLOSING)
}

slay test_websocket_performance() {
    test_start("WebSocket Performance Tests")
    
    fr fr Test large room broadcasting
    sus large_room WebSocketRoom = ws_room_create("large", "Large Room")
    
    fr fr Add many connections
    bestie i normie = 1; i <= 20; i++ {
        ws_room_join(&large_room, 2000 + i)
    }
    assert_eq_int(ws_room_get_connection_count(large_room), 20)
    
    fr fr Test broadcasting to many connections
    sus large_broadcast_count normie = ws_room_broadcast(large_room, "Broadcast to many")
    assert_eq_int(large_broadcast_count, 20)
    
    fr fr Test message queue performance
    sus conn WebSocketConnection = ws_connection_create("ws://localhost:8080/perf", cap)
    ws_connection_open(&conn)
    
    fr fr Queue many messages
    bestie i normie = 0; i < 50; i++ {
        sus msg tea = "Performance test message " + stringz.int_to_string(i)
        ws_queue_message(&conn, WS_OPCODE_TEXT, msg)
    }
    assert_eq_int(conn.queue_size, 50)
    
    fr fr Receive all messages
    bestie i normie = 0; i < 50; i++ {
        sus msg WebSocketMessage = ws_receive_message(&conn)
        assert_true(stringz.contains(msg.payload, "Performance test message"))
    }
    assert_eq_int(conn.queue_size, 0)
    
    fr fr Test frame size limits
    sus large_frame_ok lit = ws_send_text(&conn, "Small message")
    assert_eq_lit(large_frame_ok, based)
    
    fr fr Create message larger than max frame size
    sus oversized_msg tea = ""
    bestie i normie = 0; i < 10000; i++ {
        oversized_msg = stringz.concat(oversized_msg, "This is a long message segment. ")
    }
    sus large_frame_fail lit = ws_send_text(&conn, oversized_msg)
    assert_eq_lit(large_frame_fail, cap)  fr fr Should fail due to size limit
}

fr fr =============================================================================
fr fr TEST SUITE EXECUTION
fr fr =============================================================================

slay run_all_websocket_tests() {
    vibez.spill("🧪 Running WebSocket Test Suite")
    vibez.spill("===============================")
    
    fr fr Core frame handling tests
    test_websocket_frame_creation()
    test_websocket_frame_serialization()
    test_websocket_frame_parsing()
    
    fr fr Handshake and connection tests
    test_websocket_handshake()
    test_websocket_handshake_validation()
    test_websocket_connection_management()
    
    fr fr Messaging and communication tests
    test_websocket_messaging()
    test_websocket_message_queue()
    test_websocket_room_management()
    
    fr fr Client and server interface tests
    test_websocket_client()
    test_websocket_server()
    
    fr fr Advanced feature tests
    test_websocket_utility_functions()
    test_websocket_extensions()
    test_websocket_security()
    
    fr fr Integration and performance tests
    test_websocket_integration()
    test_websocket_performance()
    
    print_test_summary()
    vibez.spill("✅ WebSocket Test Suite completed!")
}

fr fr Run the complete test suite
run_all_websocket_tests()

fr fr Demo functions for manual testing
vibez.spill("")
vibez.spill("🚀 Running WebSocket Demos...")
ws_demo_client()
ws_demo_server()
ws_demo_features()
ws_demo_advanced()
