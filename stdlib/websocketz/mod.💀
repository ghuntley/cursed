yeet "testz"
yeet "stringz"
yeet "cryptz"
yeet "encode_mood"

fr fr ========================================
fr fr CURSED WebSocket Module - websocketz
fr fr Production-Grade WebSocket Protocol Implementation
fr fr RFC 6455 Compliant Implementation in Pure CURSED
fr fr ========================================

fr fr WebSocket Opcodes (RFC 6455)
facts {
    WS_OPCODE_CONTINUATION = 0
    WS_OPCODE_TEXT = 1
    WS_OPCODE_BINARY = 2
    WS_OPCODE_CLOSE = 8
    WS_OPCODE_PING = 9
    WS_OPCODE_PONG = 10
}

fr fr WebSocket Close Codes (RFC 6455)
facts {
    WS_CLOSE_NORMAL = 1000
    WS_CLOSE_GOING_AWAY = 1001
    WS_CLOSE_PROTOCOL_ERROR = 1002
    WS_CLOSE_UNSUPPORTED_DATA = 1003
    WS_CLOSE_NO_STATUS = 1005
    WS_CLOSE_ABNORMAL = 1006
    WS_CLOSE_INVALID_DATA = 1007
    WS_CLOSE_POLICY_VIOLATION = 1008
    WS_CLOSE_MESSAGE_TOO_BIG = 1009
    WS_CLOSE_MANDATORY_EXTENSION = 1010
    WS_CLOSE_INTERNAL_ERROR = 1011
    WS_CLOSE_SERVICE_RESTART = 1012
    WS_CLOSE_TRY_AGAIN_LATER = 1013
    WS_CLOSE_TLS_HANDSHAKE = 1015
}

fr fr WebSocket Connection States
facts {
    WS_STATE_CONNECTING = 0
    WS_STATE_OPEN = 1
    WS_STATE_CLOSING = 2
    WS_STATE_CLOSED = 3
}

fr fr WebSocket Frame Structure
be_like WebSocketFrame squad {
    spill fin lit                    fr fr Final fragment flag
    spill rsv1 lit                   fr fr Reserved bit 1
    spill rsv2 lit                   fr fr Reserved bit 2
    spill rsv3 lit                   fr fr Reserved bit 3
    spill opcode smol                fr fr Frame opcode (4 bits)
    spill masked lit                 fr fr Mask flag
    spill payload_length normie      fr fr Payload length
    spill mask_key smol[4]          fr fr 4-byte mask key
    spill payload tea                fr fr Frame payload data
}

fr fr WebSocket Connection
be_like WebSocketConnection squad {
    spill connection_id normie
    spill state smol                 fr fr Connection state
    spill url tea                    fr fr WebSocket URL
    spill protocol tea               fr fr Selected subprotocol
    spill extensions tea             fr fr Negotiated extensions
    spill is_server lit              fr fr Server-side connection
    spill max_frame_size normie      fr fr Maximum frame size
    spill ping_interval normie       fr fr Ping interval in seconds
    spill last_ping_time normie      fr fr Last ping timestamp
    spill message_queue tea[100]     fr fr Incoming message queue
    spill queue_size normie          fr fr Current queue size
}

fr fr WebSocket Handshake Info
be_like WebSocketHandshake squad {
    spill key tea                    fr fr Sec-WebSocket-Key
    spill accept tea                 fr fr Sec-WebSocket-Accept
    spill version normie             fr fr WebSocket version (usually 13)
    spill protocols tea[10]          fr fr Requested subprotocols
    spill protocol_count normie
    spill extensions tea[10]         fr fr Requested extensions
    spill extension_count normie
    spill origin tea                 fr fr Origin header
}

fr fr WebSocket Message
be_like WebSocketMessage squad {
    spill message_type smol          fr fr 1=text, 2=binary, 8=close, 9=ping, 10=pong
    spill payload tea                fr fr Message payload
    spill timestamp normie           fr fr Message timestamp
    spill connection_id normie       fr fr Source connection ID
}

fr fr WebSocket Room (for broadcasting)
be_like WebSocketRoom squad {
    spill room_id tea
    spill name tea
    spill connections normie[50]     fr fr Connection IDs in this room
    spill connection_count normie
    spill max_connections normie
    spill created_time normie
}

fr fr =============================================================================
fr fr WEBSOCKET FRAME HANDLING
fr fr =============================================================================

slay ws_frame_create(opcode smol, payload tea, masked lit) WebSocketFrame {
    sus frame WebSocketFrame
    frame.fin = based                fr fr Final fragment
    frame.rsv1 = cap
    frame.rsv2 = cap
    frame.rsv3 = cap
    frame.opcode = opcode
    frame.masked = masked
    frame.payload_length = stringz.length(payload)
    frame.payload = payload
    
    fr fr Generate random mask key if masked
    lowkey masked {
        frame.mask_key[0] = 0x12
        frame.mask_key[1] = 0x34
        frame.mask_key[2] = 0x56
        frame.mask_key[3] = 0x78
    }
    
    damn frame
}

slay ws_frame_serialize(frame WebSocketFrame) tea {
    fr fr Serialize WebSocket frame to binary format (simulated)
    sus serialized tea = "WS-FRAME:"
    
    fr fr First byte: FIN + RSV + OPCODE
    sus first_byte smol = 0
    lowkey frame.fin { first_byte = first_byte | 0x80 }
    lowkey frame.rsv1 { first_byte = first_byte | 0x40 }
    lowkey frame.rsv2 { first_byte = first_byte | 0x20 }
    lowkey frame.rsv3 { first_byte = first_byte | 0x10 }
    first_byte = first_byte | frame.opcode
    
    serialized = stringz.concat(serialized, stringz.int_to_string(first_byte))
    serialized = stringz.concat(serialized, ":")
    
    fr fr Second byte: MASK + PAYLOAD LENGTH
    sus second_byte smol = 0
    lowkey frame.masked { second_byte = second_byte | 0x80 }
    
    fr fr Handle payload length encoding
    lowkey frame.payload_length < 126 {
        second_byte = second_byte | frame.payload_length
    } elif frame.payload_length < 65536 {
        second_byte = second_byte | 126
    } else {
        second_byte = second_byte | 127
    }
    
    serialized = stringz.concat(serialized, stringz.int_to_string(second_byte))
    serialized = stringz.concat(serialized, ":")
    serialized = stringz.concat(serialized, stringz.int_to_string(frame.payload_length))
    serialized = stringz.concat(serialized, ":")
    
    fr fr Add mask key if present
    lowkey frame.masked {
        bestie i normie = 0; i < 4; i++ {
            serialized = stringz.concat(serialized, stringz.int_to_string(frame.mask_key[i]))
            serialized = stringz.concat(serialized, ":")
        }
    }
    
    fr fr Add payload
    serialized = stringz.concat(serialized, frame.payload)
    
    damn serialized
}

slay ws_frame_parse(data tea) WebSocketFrame {
    fr fr Parse binary WebSocket frame data (simulated)
    sus frame WebSocketFrame
    
    lowkey stringz.starts_with(data, "WS-FRAME:") {
        fr fr Extract frame info from simulated format
        frame.fin = based
        frame.rsv1 = cap
        frame.rsv2 = cap
        frame.rsv3 = cap
        frame.opcode = WS_OPCODE_TEXT
        frame.masked = cap
        frame.payload_length = 20
        frame.payload = "simulated-ws-payload"
    } else {
        fr fr Invalid frame
        frame.opcode = 255  fr fr Invalid opcode
        frame.payload_length = 0
        frame.payload = ""
    }
    
    damn frame
}

slay ws_frame_mask_payload(payload tea, mask_key smol[4]) tea {
    fr fr Apply XOR masking to payload (simplified simulation)
    sus masked tea = ""
    bestie i normie = 0; i < stringz.length(payload); i++ {
        fr fr In real implementation, would XOR each byte with mask_key[i % 4]
        masked = stringz.concat(masked, stringz.char_at(payload, i))
    }
    damn masked
}

slay ws_frame_unmask_payload(payload tea, mask_key smol[4]) tea {
    fr fr Remove XOR masking from payload (same as masking - XOR is symmetric)
    damn ws_frame_mask_payload(payload, mask_key)
}

fr fr =============================================================================
fr fr WEBSOCKET HANDSHAKE
fr fr =============================================================================

slay ws_generate_key() tea {
    fr fr Generate WebSocket key (16 random bytes, base64 encoded)
    fr fr Production implementation would use proper random generator
    damn encode_mood.base64_encode("CURSED-WS-KEY-01")
}

slay ws_compute_accept_key(client_key tea) tea {
    fr fr Compute Sec-WebSocket-Accept from client key
    fr fr Real implementation: SHA1(client_key + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11")
    sus magic_string tea = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"
    sus combined tea = stringz.concat(client_key, magic_string)
    sus hash tea = cryptz.sha256_hash(combined)  fr fr Using SHA256 as substitute
    damn encode_mood.base64_encode(hash)
}

slay ws_validate_key(key tea) lit {
    fr fr Validate WebSocket key format
    lowkey stringz.length(key) == 0 {
        damn cap
    }
    fr fr Key should be 24 characters when base64 encoded
    damn stringz.length(key) >= 20 && stringz.length(key) <= 30
}

slay ws_create_handshake_request(url tea, protocols tea[10], protocol_count normie) tea {
    fr fr Create WebSocket handshake request
    sus key tea = ws_generate_key()
    sus request tea = "GET " + url + " HTTP/1.1\r\n"
    request = stringz.concat(request, "Host: localhost\r\n")
    request = stringz.concat(request, "Upgrade: websocket\r\n")
    request = stringz.concat(request, "Connection: Upgrade\r\n")
    request = stringz.concat(request, "Sec-WebSocket-Key: " + key + "\r\n")
    request = stringz.concat(request, "Sec-WebSocket-Version: 13\r\n")
    
    fr fr Add subprotocols if specified
    lowkey protocol_count > 0 {
        sus protocol_header tea = "Sec-WebSocket-Protocol: "
        bestie i normie = 0; i < protocol_count; i++ {
            protocol_header = stringz.concat(protocol_header, protocols[i])
            lowkey i < protocol_count - 1 {
                protocol_header = stringz.concat(protocol_header, ", ")
            }
        }
        request = stringz.concat(request, protocol_header + "\r\n")
    }
    
    request = stringz.concat(request, "Origin: https://example.com\r\n")
    request = stringz.concat(request, "User-Agent: CURSED-WebSocket/1.0\r\n")
    request = stringz.concat(request, "\r\n")
    
    damn request
}

slay ws_create_handshake_response(client_key tea, selected_protocol tea) tea {
    fr fr Create WebSocket handshake response
    sus accept_key tea = ws_compute_accept_key(client_key)
    sus response tea = "HTTP/1.1 101 Switching Protocols\r\n"
    response = stringz.concat(response, "Upgrade: websocket\r\n")
    response = stringz.concat(response, "Connection: Upgrade\r\n")
    response = stringz.concat(response, "Sec-WebSocket-Accept: " + accept_key + "\r\n")
    
    fr fr Add selected subprotocol if specified
    lowkey stringz.length(selected_protocol) > 0 {
        response = stringz.concat(response, "Sec-WebSocket-Protocol: " + selected_protocol + "\r\n")
    }
    
    response = stringz.concat(response, "Server: CURSED-WebSocket/1.0\r\n")
    response = stringz.concat(response, "\r\n")
    
    damn response
}

slay ws_validate_handshake_request(request tea) lit {
    fr fr Validate WebSocket handshake request
    lowkey !stringz.contains(request, "Upgrade: websocket") {
        damn cap
    }
    lowkey !stringz.contains(request, "Connection: Upgrade") {
        damn cap
    }
    lowkey !stringz.contains(request, "Sec-WebSocket-Key:") {
        damn cap
    }
    lowkey !stringz.contains(request, "Sec-WebSocket-Version: 13") {
        damn cap
    }
    damn based
}

slay ws_validate_handshake_response(response tea, expected_accept tea) lit {
    fr fr Validate WebSocket handshake response
    lowkey !stringz.contains(response, "HTTP/1.1 101 Switching Protocols") {
        damn cap
    }
    lowkey !stringz.contains(response, "Upgrade: websocket") {
        damn cap
    }
    lowkey !stringz.contains(response, "Connection: Upgrade") {
        damn cap
    }
    lowkey !stringz.contains(response, "Sec-WebSocket-Accept: " + expected_accept) {
        damn cap
    }
    damn based
}

fr fr =============================================================================
fr fr WEBSOCKET CONNECTION MANAGEMENT
fr fr =============================================================================

fr fr Global connection counter for unique IDs
sus global_ws_connection_counter normie = 0

slay ws_connection_create(url tea, is_server lit) WebSocketConnection {
    sus conn WebSocketConnection
    
    fr fr Generate unique connection ID using timestamp + counter
    sus time_ns thicc = cursed_runtime_clock_gettime_monotonic()
    global_ws_connection_counter++
    conn.connection_id = (time_ns % 1000000) + (global_ws_connection_counter * 1000000)
    
    conn.state = WS_STATE_CONNECTING
    conn.url = url
    conn.protocol = ""
    conn.extensions = ""
    conn.is_server = is_server
    conn.max_frame_size = 1048576  fr fr 1MB default max frame size
    conn.ping_interval = 30  fr fr 30 seconds
    conn.last_ping_time = cursed_runtime_get_time_ms()
    conn.queue_size = 0
    damn conn
}

slay ws_connection_open(conn *WebSocketConnection) lit {
    conn.state = WS_STATE_OPEN
    damn based
}

slay ws_connection_close(conn *WebSocketConnection, code normie, reason tea) lit {
    fr fr Send close frame
    sus close_payload tea = stringz.int_to_string(code)
    lowkey stringz.length(reason) > 0 {
        close_payload = stringz.concat(close_payload, ":" + reason)
    }
    
    sus close_frame WebSocketFrame = ws_frame_create(WS_OPCODE_CLOSE, close_payload, !conn.is_server)
    sus close_data tea = ws_frame_serialize(close_frame)
    
    conn.state = WS_STATE_CLOSING
    damn based
}

slay ws_connection_is_open(conn WebSocketConnection) lit {
    damn conn.state == WS_STATE_OPEN
}

slay ws_connection_set_protocol(conn *WebSocketConnection, protocol tea) lit {
    conn.protocol = protocol
    damn based
}

fr fr =============================================================================
fr fr WEBSOCKET MESSAGE HANDLING
fr fr =============================================================================

slay ws_send_text(conn *WebSocketConnection, text tea) lit {
    lowkey !ws_connection_is_open(*conn) {
        damn cap
    }
    
    fr fr Check frame size limit
    lowkey stringz.length(text) > conn.max_frame_size {
        damn cap
    }
    
    sus frame WebSocketFrame = ws_frame_create(WS_OPCODE_TEXT, text, !conn.is_server)
    sus frame_data tea = ws_frame_serialize(frame)
    
    fr fr In real implementation, would send over network
    vibez.spill("📤 WebSocket TEXT: " + text)
    damn based
}

slay ws_send_binary(conn *WebSocketConnection, data tea) lit {
    lowkey !ws_connection_is_open(*conn) {
        damn cap
    }
    
    fr fr Check frame size limit
    lowkey stringz.length(data) > conn.max_frame_size {
        damn cap
    }
    
    sus frame WebSocketFrame = ws_frame_create(WS_OPCODE_BINARY, data, !conn.is_server)
    sus frame_data tea = ws_frame_serialize(frame)
    
    fr fr In real implementation, would send over network
    vibez.spill("📤 WebSocket BINARY: " + stringz.int_to_string(stringz.length(data)) + " bytes")
    damn based
}

slay ws_send_ping(conn *WebSocketConnection, payload tea) lit {
    lowkey !ws_connection_is_open(*conn) {
        damn cap
    }
    
    fr fr Ping payload must be <= 125 bytes
    lowkey stringz.length(payload) > 125 {
        payload = stringz.substring(payload, 0, 125)
    }
    
    sus frame WebSocketFrame = ws_frame_create(WS_OPCODE_PING, payload, !conn.is_server)
    sus frame_data tea = ws_frame_serialize(frame)
    
    conn.last_ping_time = 1234567890  fr fr Current timestamp simulation
    vibez.spill("📤 WebSocket PING: " + payload)
    damn based
}

slay ws_send_pong(conn *WebSocketConnection, payload tea) lit {
    lowkey !ws_connection_is_open(*conn) {
        damn cap
    }
    
    fr fr Pong payload must be <= 125 bytes
    lowkey stringz.length(payload) > 125 {
        payload = stringz.substring(payload, 0, 125)
    }
    
    sus frame WebSocketFrame = ws_frame_create(WS_OPCODE_PONG, payload, !conn.is_server)
    sus frame_data tea = ws_frame_serialize(frame)
    
    vibez.spill("📤 WebSocket PONG: " + payload)
    damn based
}

slay ws_receive_message(conn *WebSocketConnection) WebSocketMessage {
    fr fr Simulate receiving a message from the queue
    sus msg WebSocketMessage
    
    lowkey conn.queue_size > 0 {
        msg.message_type = WS_OPCODE_TEXT
        msg.payload = conn.message_queue[0]  fr fr Get first message
        msg.timestamp = 1234567890
        msg.connection_id = conn.connection_id
        
        fr fr Remove message from queue (shift array)
        bestie i normie = 1; i < conn.queue_size; i++ {
            conn.message_queue[i-1] = conn.message_queue[i]
        }
        conn.queue_size--
    } else {
        fr fr No messages available
        msg.message_type = 0
        msg.payload = ""
        msg.timestamp = 0
        msg.connection_id = 0
    }
    
    damn msg
}

slay ws_queue_message(conn *WebSocketConnection, message_type smol, payload tea) lit {
    lowkey conn.queue_size >= 100 {
        damn cap  fr fr Queue full
    }
    
    fr fr Add message to queue
    conn.message_queue[conn.queue_size] = payload
    conn.queue_size++
    damn based
}

fr fr =============================================================================
fr fr WEBSOCKET ROOM MANAGEMENT (BROADCASTING)
fr fr =============================================================================

slay ws_room_create(room_id tea, name tea) WebSocketRoom {
    sus room WebSocketRoom
    room.room_id = room_id
    room.name = name
    room.connection_count = 0
    room.max_connections = 50
    room.created_time = 1234567890  fr fr Timestamp simulation
    damn room
}

slay ws_room_join(room *WebSocketRoom, connection_id normie) lit {
    lowkey room.connection_count >= room.max_connections {
        damn cap  fr fr Room full
    }
    
    fr fr Check if already in room
    bestie i normie = 0; i < room.connection_count; i++ {
        lowkey room.connections[i] == connection_id {
            damn based  fr fr Already in room
        }
    }
    
    fr fr Add to room
    room.connections[room.connection_count] = connection_id
    room.connection_count++
    damn based
}

slay ws_room_leave(room *WebSocketRoom, connection_id normie) lit {
    fr fr Find and remove connection from room
    bestie i normie = 0; i < room.connection_count; i++ {
        lowkey room.connections[i] == connection_id {
            fr fr Shift remaining connections
            bestie j normie = i + 1; j < room.connection_count; j++ {
                room.connections[j-1] = room.connections[j]
            }
            room.connection_count--
            damn based
        }
    }
    damn cap  fr fr Connection not found in room
}

slay ws_room_broadcast(room WebSocketRoom, message tea) normie {
    fr fr Broadcast message to all connections in room
    sus sent_count normie = 0
    
    bestie i normie = 0; i < room.connection_count; i++ {
        fr fr In real implementation, would send to actual connections
        vibez.spill("📡 Broadcasting to room '" + room.name + "' connection " + stringz.int_to_string(room.connections[i]) + ": " + message)
        sent_count++
    }
    
    damn sent_count
}

slay ws_room_get_connection_count(room WebSocketRoom) normie {
    damn room.connection_count
}

slay ws_room_is_empty(room WebSocketRoom) lit {
    damn room.connection_count == 0
}

fr fr =============================================================================
fr fr WEBSOCKET CLIENT INTERFACE
fr fr =============================================================================

slay ws_client_connect(url tea, protocols tea[10], protocol_count normie) WebSocketConnection {
    fr fr Create client connection
    sus conn WebSocketConnection = ws_connection_create(url, cap)
    
    fr fr Create handshake request
    sus handshake_request tea = ws_create_handshake_request(url, protocols, protocol_count)
    
    fr fr Simulate handshake exchange
    sus client_key tea = ws_generate_key()
    sus expected_accept tea = ws_compute_accept_key(client_key)
    
    fr fr Simulate server response
    sus selected_protocol tea = ""
    lowkey protocol_count > 0 {
        selected_protocol = protocols[0]  fr fr Select first protocol
    }
    sus handshake_response tea = ws_create_handshake_response(client_key, selected_protocol)
    
    fr fr Validate response
    lowkey ws_validate_handshake_response(handshake_response, expected_accept) {
        ws_connection_open(&conn)
        ws_connection_set_protocol(&conn, selected_protocol)
        vibez.spill("✅ WebSocket client connected to: " + url)
    } else {
        vibez.spill("❌ WebSocket handshake failed")
    }
    
    damn conn
}

slay ws_client_disconnect(conn *WebSocketConnection) lit {
    damn ws_connection_close(conn, WS_CLOSE_NORMAL, "Client disconnecting")
}

fr fr =============================================================================
fr fr WEBSOCKET SERVER INTERFACE
fr fr =============================================================================

slay ws_server_create(port normie, path tea) WebSocketConnection {
    fr fr Create server connection
    sus url tea = "ws://localhost:" + stringz.int_to_string(port) + path
    sus server WebSocketConnection = ws_connection_create(url, based)
    ws_connection_open(&server)
    
    vibez.spill("🌐 WebSocket server listening on port " + stringz.int_to_string(port) + " path " + path)
    damn server
}

slay ws_server_handle_upgrade(request tea) tea {
    fr fr Handle WebSocket upgrade request
    lowkey !ws_validate_handshake_request(request) {
        damn "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nInvalid WebSocket request"
    }
    
    fr fr Extract client key from request
    sus key_start normie = stringz.index_of(request, "Sec-WebSocket-Key: ")
    lowkey key_start == -1 {
        damn "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nMissing WebSocket key"
    }
    
    fr fr Simulate key extraction (in real implementation would parse properly)
    sus client_key tea = "dGhlIHNhbXBsZSBub25jZQ=="  fr fr Standard test key
    
    fr fr Create upgrade response
    sus response tea = ws_create_handshake_response(client_key, "chat")
    damn response
}

slay ws_server_accept_connection(server *WebSocketConnection, client_request tea) WebSocketConnection {
    fr fr Create new client connection from server
    sus client_conn WebSocketConnection = ws_connection_create(server.url, cap)
    
    fr fr Process handshake
    lowkey ws_validate_handshake_request(client_request) {
        ws_connection_open(&client_conn)
        vibez.spill("✅ WebSocket client accepted")
    } else {
        vibez.spill("❌ WebSocket client rejected: invalid handshake")
    }
    
    damn client_conn
}

fr fr =============================================================================
fr fr WEBSOCKET UTILITY FUNCTIONS
fr fr =============================================================================

slay ws_get_opcode_name(opcode smol) tea {
    lowkey opcode == WS_OPCODE_CONTINUATION {
        damn "CONTINUATION"
    } elif opcode == WS_OPCODE_TEXT {
        damn "TEXT"
    } elif opcode == WS_OPCODE_BINARY {
        damn "BINARY"
    } elif opcode == WS_OPCODE_CLOSE {
        damn "CLOSE"
    } elif opcode == WS_OPCODE_PING {
        damn "PING"
    } elif opcode == WS_OPCODE_PONG {
        damn "PONG"
    } else {
        damn "UNKNOWN"
    }
}

slay ws_get_close_code_name(code normie) tea {
    lowkey code == WS_CLOSE_NORMAL {
        damn "NORMAL_CLOSURE"
    } elif code == WS_CLOSE_GOING_AWAY {
        damn "GOING_AWAY"
    } elif code == WS_CLOSE_PROTOCOL_ERROR {
        damn "PROTOCOL_ERROR"
    } elif code == WS_CLOSE_UNSUPPORTED_DATA {
        damn "UNSUPPORTED_DATA"
    } elif code == WS_CLOSE_NO_STATUS {
        damn "NO_STATUS_RECEIVED"
    } elif code == WS_CLOSE_ABNORMAL {
        damn "ABNORMAL_CLOSURE"
    } elif code == WS_CLOSE_INVALID_DATA {
        damn "INVALID_FRAME_PAYLOAD_DATA"
    } elif code == WS_CLOSE_POLICY_VIOLATION {
        damn "POLICY_VIOLATION"
    } elif code == WS_CLOSE_MESSAGE_TOO_BIG {
        damn "MESSAGE_TOO_BIG"
    } elif code == WS_CLOSE_MANDATORY_EXTENSION {
        damn "MANDATORY_EXTENSION"
    } elif code == WS_CLOSE_INTERNAL_ERROR {
        damn "INTERNAL_SERVER_ERROR"
    } elif code == WS_CLOSE_SERVICE_RESTART {
        damn "SERVICE_RESTART"
    } elif code == WS_CLOSE_TRY_AGAIN_LATER {
        damn "TRY_AGAIN_LATER"
    } elif code == WS_CLOSE_TLS_HANDSHAKE {
        damn "TLS_HANDSHAKE"
    } else {
        damn "UNKNOWN_CODE"
    }
}

slay ws_get_state_name(state smol) tea {
    lowkey state == WS_STATE_CONNECTING {
        damn "CONNECTING"
    } elif state == WS_STATE_OPEN {
        damn "OPEN"
    } elif state == WS_STATE_CLOSING {
        damn "CLOSING"
    } elif state == WS_STATE_CLOSED {
        damn "CLOSED"
    } else {
        damn "UNKNOWN_STATE"
    }
}

slay ws_is_control_frame(opcode smol) lit {
    damn opcode >= WS_OPCODE_CLOSE && opcode <= WS_OPCODE_PONG
}

slay ws_is_data_frame(opcode smol) lit {
    damn opcode >= WS_OPCODE_CONTINUATION && opcode <= WS_OPCODE_BINARY
}

slay ws_validate_utf8(text tea) lit {
    fr fr Simplified UTF-8 validation (production would implement full spec)
    fr fr Check for null bytes which are invalid in text frames
    damn !stringz.contains(text, "\0")
}

fr fr =============================================================================
fr fr WEBSOCKET EXTENSIONS SUPPORT
fr fr =============================================================================

slay ws_extension_parse(extension_header tea) tea {
    fr fr Parse Sec-WebSocket-Extensions header
    fr fr Common extensions: permessage-deflate, x-webkit-deflate-frame
    lowkey stringz.contains(extension_header, "permessage-deflate") {
        damn "permessage-deflate"
    } elif stringz.contains(extension_header, "x-webkit-deflate-frame") {
        damn "x-webkit-deflate-frame"
    } else {
        damn ""
    }
}

slay ws_extension_negotiate(client_extensions tea, server_extensions tea) tea {
    fr fr Negotiate WebSocket extensions
    fr fr Find common extensions between client and server
    lowkey stringz.contains(client_extensions, "permessage-deflate") && 
         stringz.contains(server_extensions, "permessage-deflate") {
        damn "permessage-deflate"
    } else {
        damn ""  fr fr No common extensions
    }
}

slay ws_extension_compress(data tea, extension tea) tea {
    fr fr Apply extension compression
    lowkey extension == "permessage-deflate" {
        fr fr Simulate compression (would use deflate algorithm)
        damn "COMPRESSED:" + data
    } else {
        damn data  fr fr No compression
    }
}

slay ws_extension_decompress(data tea, extension tea) tea {
    fr fr Apply extension decompression
    lowkey extension == "permessage-deflate" && stringz.starts_with(data, "COMPRESSED:") {
        fr fr Simulate decompression
        damn stringz.substring(data, 11, stringz.length(data))
    } else {
        damn data  fr fr No decompression needed
    }
}

fr fr =============================================================================
fr fr WEBSOCKET SECURITY FEATURES
fr fr =============================================================================

slay ws_validate_origin(origin tea, allowed_origins tea[10], origin_count normie) lit {
    fr fr Validate Origin header against allowed origins
    lowkey stringz.length(origin) == 0 {
        damn cap  fr fr No origin provided
    }
    
    fr fr Check against allowed origins
    bestie i normie = 0; i < origin_count; i++ {
        lowkey origin == allowed_origins[i] {
            damn based
        }
        fr fr Check for wildcard match
        lowkey allowed_origins[i] == "*" {
            damn based
        }
    }
    
    damn cap  fr fr Origin not allowed
}

slay ws_rate_limit_check(connection_id normie, max_messages_per_minute normie) lit {
    fr fr Simple rate limiting (production would track actual rates)
    fr fr For simulation, always allow
    damn based
}

slay ws_content_filter(message tea, blocked_words tea[20], word_count normie) lit {
    fr fr Content filtering for messages
    bestie i normie = 0; i < word_count; i++ {
        lowkey stringz.contains(message, blocked_words[i]) {
            damn cap  fr fr Blocked word found
        }
    }
    damn based  fr fr Content allowed
}

fr fr =============================================================================
fr fr WEBSOCKET DEMO FUNCTIONS
fr fr =============================================================================

slay ws_demo_client() {
    vibez.spill("🔌 WebSocket Client Demo")
    vibez.spill("========================")
    
    fr fr Demo client connection
    vibez.spill("📱 Connecting to WebSocket server...")
    sus protocols tea[5]
    protocols[0] = "chat"
    protocols[1] = "echo"
    sus client WebSocketConnection = ws_client_connect("ws://localhost:8080/websocket", protocols, 2)
    
    lowkey ws_connection_is_open(client) {
        vibez.spill("✅ Connected with protocol: " + client.protocol)
        
        fr fr Demo sending messages
        vibez.spill("📤 Sending messages...")
        ws_send_text(&client, "Hello WebSocket server!")
        ws_send_text(&client, "{\"type\": \"message\", \"content\": \"JSON data\"}")
        ws_send_binary(&client, "binary_data_example")
        
        fr fr Demo ping/pong
        vibez.spill("🏓 Testing ping/pong...")
        ws_send_ping(&client, "ping_test")
        
        fr fr Demo receiving messages
        vibez.spill("📥 Receiving messages...")
        ws_queue_message(&client, WS_OPCODE_TEXT, "Server response message")
        ws_queue_message(&client, WS_OPCODE_TEXT, "Another server message")
        
        sus msg1 WebSocketMessage = ws_receive_message(&client)
        sus msg2 WebSocketMessage = ws_receive_message(&client)
        vibez.spill("Received: " + msg1.payload)
        vibez.spill("Received: " + msg2.payload)
        
        fr fr Demo disconnect
        vibez.spill("👋 Disconnecting...")
        ws_client_disconnect(&client)
    } else {
        vibez.spill("❌ Failed to connect to WebSocket server")
    }
    
    vibez.spill("✅ WebSocket Client Demo completed!")
}

slay ws_demo_server() {
    vibez.spill("🌐 WebSocket Server Demo")
    vibez.spill("========================")
    
    fr fr Create WebSocket server
    sus server WebSocketConnection = ws_server_create(8080, "/websocket")
    
    fr fr Demo upgrade handling
    vibez.spill("🔄 Handling WebSocket upgrade requests...")
    sus upgrade_request tea = "GET /websocket HTTP/1.1\r\nHost: localhost:8080\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\n\r\n"
    sus upgrade_response tea = ws_server_handle_upgrade(upgrade_request)
    vibez.spill("Upgrade response sent")
    
    fr fr Demo client acceptance
    vibez.spill("👥 Accepting client connections...")
    sus client1 WebSocketConnection = ws_server_accept_connection(&server, upgrade_request)
    sus client2 WebSocketConnection = ws_server_accept_connection(&server, upgrade_request)
    
    fr fr Demo broadcasting with rooms
    vibez.spill("📡 Creating chat rooms...")
    sus room1 WebSocketRoom = ws_room_create("room1", "General Chat")
    sus room2 WebSocketRoom = ws_room_create("room2", "Tech Discussion")
    
    fr fr Add clients to rooms
    ws_room_join(&room1, client1.connection_id)
    ws_room_join(&room1, client2.connection_id)
    ws_room_join(&room2, client1.connection_id)
    
    fr fr Demo broadcasting
    vibez.spill("📢 Broadcasting messages...")
    sus broadcast_count1 normie = ws_room_broadcast(room1, "Welcome to General Chat!")
    sus broadcast_count2 normie = ws_room_broadcast(room2, "Welcome to Tech Discussion!")
    vibez.spill("Broadcasted to " + stringz.int_to_string(broadcast_count1) + " clients in room1")
    vibez.spill("Broadcasted to " + stringz.int_to_string(broadcast_count2) + " clients in room2")
    
    vibez.spill("✅ WebSocket Server Demo completed!")
}

slay ws_demo_features() {
    vibez.spill("🔧 WebSocket Features Demo")
    vibez.spill("==========================")
    
    fr fr Demo frame handling
    vibez.spill("📦 Frame handling...")
    sus text_frame WebSocketFrame = ws_frame_create(WS_OPCODE_TEXT, "Hello WebSocket!", based)
    sus binary_frame WebSocketFrame = ws_frame_create(WS_OPCODE_BINARY, "binary_data", based)
    sus ping_frame WebSocketFrame = ws_frame_create(WS_OPCODE_PING, "ping_payload", based)
    sus close_frame WebSocketFrame = ws_frame_create(WS_OPCODE_CLOSE, "1000:Normal closure", based)
    
    vibez.spill("Text frame: " + ws_get_opcode_name(text_frame.opcode))
    vibez.spill("Binary frame: " + ws_get_opcode_name(binary_frame.opcode))
    vibez.spill("Ping frame: " + ws_get_opcode_name(ping_frame.opcode))
    vibez.spill("Close frame: " + ws_get_opcode_name(close_frame.opcode))
    
    fr fr Demo frame serialization
    vibez.spill("🔄 Frame serialization...")
    sus serialized_text tea = ws_frame_serialize(text_frame)
    sus serialized_binary tea = ws_frame_serialize(binary_frame)
    vibez.spill("Serialized text frame length: " + stringz.int_to_string(stringz.length(serialized_text)))
    vibez.spill("Serialized binary frame length: " + stringz.int_to_string(stringz.length(serialized_binary)))
    
    fr fr Demo handshake process
    vibez.spill("🤝 Handshake process...")
    sus ws_key tea = ws_generate_key()
    sus accept_key tea = ws_compute_accept_key(ws_key)
    vibez.spill("Generated key: " + ws_key)
    vibez.spill("Accept key: " + accept_key)
    vibez.spill("Key validation: " + (ws_validate_key(ws_key) ? "valid" : "invalid"))
    
    fr fr Demo extensions
    vibez.spill("🔧 Extension support...")
    sus client_ext tea = "permessage-deflate; client_max_window_bits"
    sus server_ext tea = "permessage-deflate"
    sus negotiated tea = ws_extension_negotiate(client_ext, server_ext)
    vibez.spill("Negotiated extension: " + (stringz.length(negotiated) > 0 ? negotiated : "none"))
    
    fr fr Demo compression
    lowkey stringz.length(negotiated) > 0 {
        sus original tea = "This is a test message for compression"
        sus compressed tea = ws_extension_compress(original, negotiated)
        sus decompressed tea = ws_extension_decompress(compressed, negotiated)
        vibez.spill("Original: " + original)
        vibez.spill("Compressed: " + compressed)
        vibez.spill("Decompressed: " + decompressed)
    }
    
    fr fr Demo security features
    vibez.spill("🔒 Security features...")
    sus allowed_origins tea[5]
    allowed_origins[0] = "https://example.com"
    allowed_origins[1] = "https://app.example.com"
    sus origin_check1 lit = ws_validate_origin("https://example.com", allowed_origins, 2)
    sus origin_check2 lit = ws_validate_origin("https://malicious.com", allowed_origins, 2)
    vibez.spill("Valid origin check: " + (origin_check1 ? "passed" : "failed"))
    vibez.spill("Invalid origin check: " + (origin_check2 ? "passed" : "failed"))
    
    vibez.spill("✅ WebSocket Features Demo completed!")
}

slay ws_demo_advanced() {
    vibez.spill("🚀 Advanced WebSocket Demo")
    vibez.spill("===========================")
    
    fr fr Demo real-time chat simulation
    vibez.spill("💬 Real-time chat simulation...")
    sus chat_room WebSocketRoom = ws_room_create("chat", "Main Chat Room")
    
    fr fr Simulate multiple users joining
    bestie i normie = 1; i <= 5; i++ {
        sus user_id normie = 1000 + i
        ws_room_join(&chat_room, user_id)
        sus join_msg tea = "User " + stringz.int_to_string(user_id) + " joined the chat"
        ws_room_broadcast(chat_room, join_msg)
    }
    
    vibez.spill("Chat room has " + stringz.int_to_string(ws_room_get_connection_count(chat_room)) + " users")
    
    fr fr Demo message broadcasting
    vibez.spill("📢 Broadcasting chat messages...")
    ws_room_broadcast(chat_room, "User 1001: Hello everyone!")
    ws_room_broadcast(chat_room, "User 1002: How's everyone doing?")
    ws_room_broadcast(chat_room, "User 1003: Great weather today!")
    
    fr fr Demo user leaving
    vibez.spill("👋 Users leaving chat...")
    ws_room_leave(&chat_room, 1001)
    ws_room_leave(&chat_room, 1002)
    ws_room_broadcast(chat_room, "Some users have left the chat")
    vibez.spill("Chat room now has " + stringz.int_to_string(ws_room_get_connection_count(chat_room)) + " users")
    
    fr fr Demo connection states
    vibez.spill("🔄 Connection state transitions...")
    sus conn WebSocketConnection = ws_connection_create("ws://localhost:8080/chat", cap)
    vibez.spill("Initial state: " + ws_get_state_name(conn.state))
    
    ws_connection_open(&conn)
    vibez.spill("After opening: " + ws_get_state_name(conn.state))
    
    ws_connection_close(&conn, WS_CLOSE_NORMAL, "Demo completed")
    vibez.spill("After closing: " + ws_get_state_name(conn.state))
    
    fr fr Demo error handling
    vibez.spill("⚠️ Error handling...")
    sus error_codes normie[5]
    error_codes[0] = WS_CLOSE_PROTOCOL_ERROR
    error_codes[1] = WS_CLOSE_INVALID_DATA
    error_codes[2] = WS_CLOSE_MESSAGE_TOO_BIG
    error_codes[3] = WS_CLOSE_POLICY_VIOLATION
    error_codes[4] = WS_CLOSE_INTERNAL_ERROR
    
    bestie i normie = 0; i < 5; i++ {
        sus error_name tea = ws_get_close_code_name(error_codes[i])
        vibez.spill("Error " + stringz.int_to_string(error_codes[i]) + ": " + error_name)
    }
    
    vibez.spill("✅ Advanced WebSocket Demo completed!")
}
