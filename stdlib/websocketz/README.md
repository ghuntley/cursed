# CURSED WebSocket Module (websocketz)

Modern WebSocket protocol implementation for CURSED with real-time communication, broadcasting, and RFC 6455 compliance.

## Overview

The `websocketz` module provides a complete WebSocket implementation following RFC 6455 specifications. It includes client and server capabilities, frame processing, handshake handling, message broadcasting, room management, and advanced features like extensions and security.

## Features

### ✅ Core WebSocket Protocol
- **Frame Processing**: Complete WebSocket frame handling (TEXT, BINARY, CLOSE, PING, PONG)
- **Handshake Protocol**: RFC 6455 compliant handshake with key generation and validation
- **Message Types**: Support for text, binary, and control frames
- **Connection Management**: Full connection lifecycle with proper state transitions
- **Masking**: Client-side frame masking as required by specification

### ✅ Real-Time Communication
- **Bidirectional Messaging**: Full-duplex communication between client and server
- **Message Queue**: Asynchronous message handling with queuing
- **Broadcasting**: Room-based message broadcasting to multiple clients
- **Ping/Pong**: Connection health monitoring and keep-alive
- **Large Message Support**: Handling of large payloads with size limits

### ✅ Advanced Features
- **Room Management**: Multi-room broadcasting with join/leave functionality
- **Extensions**: Support for compression extensions (permessage-deflate)
- **Security**: Origin validation, rate limiting, and content filtering
- **Subprotocols**: Negotiation and selection of WebSocket subprotocols
- **Connection States**: Proper state machine implementation

### ✅ Client & Server APIs
- **High-Level APIs**: Simple WebSocket client and server interfaces
- **Event-Driven**: Message-based communication patterns
- **Streaming**: Support for continuous data streaming
- **Error Handling**: Comprehensive error codes and graceful closure

## Quick Start

### WebSocket Client

```cursed
yeet "websocketz"

# Connect to WebSocket server
sus protocols [2]tea
protocols[0] = "chat"
protocols[1] = "echo"

sus client WebSocketConnection = ws_client_connect("ws://localhost:8080/websocket", protocols, 2)

lowkey ws_connection_is_open(client) {
    # Send messages
    ws_send_text(&client, "Hello WebSocket server!")
    ws_send_text(&client, "{\"type\": \"message\", \"content\": \"JSON data\"}")
    
    # Send binary data
    ws_send_binary(&client, "binary_data_example")
    
    # Send ping
    ws_send_ping(&client, "ping_payload")
    
    # Receive messages
    sus msg WebSocketMessage = ws_receive_message(&client)
    vibez.spill("Received: " + msg.payload)
    
    # Disconnect
    ws_client_disconnect(&client)
}
```

### WebSocket Server

```cursed
yeet "websocketz"

# Create WebSocket server
sus server WebSocketConnection = ws_server_create(8080, "/websocket")

# Handle upgrade requests
sus upgrade_request tea = "GET /websocket HTTP/1.1\r\nUpgrade: websocket\r\n..."
sus upgrade_response tea = ws_server_handle_upgrade(upgrade_request)
vibez.spill("Upgrade response sent")

# Accept client connections
sus client_conn WebSocketConnection = ws_server_accept_connection(&server, upgrade_request)

lowkey ws_connection_is_open(client_conn) {
    # Handle client messages
    ws_queue_message(&client_conn, WS_OPCODE_TEXT, "Welcome to the server!")
    
    # Send response
    ws_send_text(&client_conn, "Server response message")
    
    vibez.spill("Client connected and handled")
}
```

### Room-Based Broadcasting

```cursed
yeet "websocketz"

# Create chat room
sus room WebSocketRoom = ws_room_create("chat_room", "General Chat")

# Add clients to room
ws_room_join(&room, 1001)  # Client ID 1001
ws_room_join(&room, 1002)  # Client ID 1002
ws_room_join(&room, 1003)  # Client ID 1003

# Broadcast message to all clients in room
sus broadcast_count normie = ws_room_broadcast(room, "Welcome everyone!")
vibez.spill("Broadcasted to " + stringz.int_to_string(broadcast_count) + " clients")

# Remove client from room
ws_room_leave(&room, 1002)

# Check room status
lowkey !ws_room_is_empty(room) {
    vibez.spill("Room has " + stringz.int_to_string(ws_room_get_connection_count(room)) + " clients")
}
```

### Advanced Usage

```cursed
yeet "websocketz"

# Create connection with custom settings
sus conn WebSocketConnection = ws_connection_create("ws://localhost:8080/advanced", cap)
conn.max_frame_size = 2097152  # 2MB max frame size
conn.ping_interval = 15        # 15 second ping interval

ws_connection_open(&conn)

# Handle different message types
ws_queue_message(&conn, WS_OPCODE_TEXT, "Text message")
ws_queue_message(&conn, WS_OPCODE_BINARY, "binary_data")

bestie {
    sus msg WebSocketMessage = ws_receive_message(&conn)
    lowkey msg.message_type == 0 {
        break  # No more messages
    }
    
    lowkey msg.message_type == WS_OPCODE_TEXT {
        vibez.spill("Text: " + msg.payload)
    } elif msg.message_type == WS_OPCODE_BINARY {
        vibez.spill("Binary: " + stringz.int_to_string(stringz.length(msg.payload)) + " bytes")
    }
}

# Close with custom code and reason
ws_connection_close(&conn, WS_CLOSE_NORMAL, "Session completed")
```

## API Reference

### Connection Management

#### `ws_connection_create(url tea, is_server lit) -> WebSocketConnection`
Creates a new WebSocket connection for client or server use.

#### `ws_connection_open(conn *WebSocketConnection) -> lit`
Opens a WebSocket connection and sets state to OPEN.

#### `ws_connection_close(conn *WebSocketConnection, code normie, reason tea) -> lit`
Closes a WebSocket connection with specified close code and reason.

#### `ws_connection_is_open(conn WebSocketConnection) -> lit`
Checks if a WebSocket connection is in the OPEN state.

### Frame Processing

#### `ws_frame_create(opcode smol, payload tea, masked lit) -> WebSocketFrame`
Creates a WebSocket frame with specified opcode, payload, and masking.

#### `ws_frame_serialize(frame WebSocketFrame) -> tea`
Serializes a WebSocket frame to binary format for transmission.

#### `ws_frame_parse(data tea) -> WebSocketFrame`
Parses binary data into a WebSocket frame structure.

### Handshake Handling

#### `ws_generate_key() -> tea`
Generates a random WebSocket key for handshake requests.

#### `ws_compute_accept_key(client_key tea) -> tea`
Computes the Sec-WebSocket-Accept value from client key.

#### `ws_create_handshake_request(url tea, protocols [10]tea, protocol_count normie) -> tea`
Creates a complete WebSocket handshake request.

#### `ws_create_handshake_response(client_key tea, selected_protocol tea) -> tea`
Creates a WebSocket handshake response with accept key.

#### `ws_validate_handshake_request(request tea) -> lit`
Validates incoming WebSocket handshake request.

#### `ws_validate_handshake_response(response tea, expected_accept tea) -> lit`
Validates WebSocket handshake response from server.

### Messaging

#### `ws_send_text(conn *WebSocketConnection, text tea) -> lit`
Sends a text message over the WebSocket connection.

#### `ws_send_binary(conn *WebSocketConnection, data tea) -> lit`
Sends binary data over the WebSocket connection.

#### `ws_send_ping(conn *WebSocketConnection, payload tea) -> lit`
Sends a PING frame with optional payload.

#### `ws_send_pong(conn *WebSocketConnection, payload tea) -> lit`
Sends a PONG frame in response to PING.

#### `ws_receive_message(conn *WebSocketConnection) -> WebSocketMessage`
Receives the next message from the connection queue.

#### `ws_queue_message(conn *WebSocketConnection, message_type smol, payload tea) -> lit`
Queues an incoming message for later retrieval.

### Client API

#### `ws_client_connect(url tea, protocols [10]tea, protocol_count normie) -> WebSocketConnection`
Connects to a WebSocket server with optional subprotocols.

#### `ws_client_disconnect(conn *WebSocketConnection) -> lit`
Disconnects from WebSocket server with normal closure.

### Server API

#### `ws_server_create(port normie, path tea) -> WebSocketConnection`
Creates a WebSocket server listening on specified port and path.

#### `ws_server_handle_upgrade(request tea) -> tea`
Handles WebSocket upgrade request and returns response.

#### `ws_server_accept_connection(server *WebSocketConnection, client_request tea) -> WebSocketConnection`
Accepts a new client connection after successful handshake.

### Room Management

#### `ws_room_create(room_id tea, name tea) -> WebSocketRoom`
Creates a new WebSocket room for broadcasting.

#### `ws_room_join(room *WebSocketRoom, connection_id normie) -> lit`
Adds a connection to a WebSocket room.

#### `ws_room_leave(room *WebSocketRoom, connection_id normie) -> lit`
Removes a connection from a WebSocket room.

#### `ws_room_broadcast(room WebSocketRoom, message tea) -> normie`
Broadcasts a message to all connections in the room.

#### `ws_room_get_connection_count(room WebSocketRoom) -> normie`
Returns the number of connections in the room.

#### `ws_room_is_empty(room WebSocketRoom) -> lit`
Checks if the room has no connections.

### Extensions

#### `ws_extension_parse(extension_header tea) -> tea`
Parses Sec-WebSocket-Extensions header to identify supported extensions.

#### `ws_extension_negotiate(client_extensions tea, server_extensions tea) -> tea`
Negotiates common extensions between client and server.

#### `ws_extension_compress(data tea, extension tea) -> tea`
Applies extension compression to message data.

#### `ws_extension_decompress(data tea, extension tea) -> tea`
Applies extension decompression to message data.

### Security

#### `ws_validate_origin(origin tea, allowed_origins [10]tea, origin_count normie) -> lit`
Validates request origin against allowed origins list.

#### `ws_rate_limit_check(connection_id normie, max_messages_per_minute normie) -> lit`
Checks if connection is within rate limits.

#### `ws_content_filter(message tea, blocked_words [20]tea, word_count normie) -> lit`
Filters message content for blocked words or phrases.

### Utility Functions

#### `ws_get_opcode_name(opcode smol) -> tea`
Returns human-readable name for WebSocket opcode.

#### `ws_get_close_code_name(code normie) -> tea`
Returns human-readable name for WebSocket close code.

#### `ws_get_state_name(state smol) -> tea`
Returns human-readable name for connection state.

#### `ws_is_control_frame(opcode smol) -> lit`
Checks if opcode represents a control frame (CLOSE, PING, PONG).

#### `ws_is_data_frame(opcode smol) -> lit`
Checks if opcode represents a data frame (TEXT, BINARY, CONTINUATION).

## WebSocket Opcodes

| Opcode | Value | Description |
|--------|-------|-------------|
| CONTINUATION | 0 | Continuation frame |
| TEXT | 1 | Text data frame |
| BINARY | 2 | Binary data frame |
| CLOSE | 8 | Connection close |
| PING | 9 | Ping frame |
| PONG | 10 | Pong frame |

## WebSocket Close Codes

| Close Code | Value | Description |
|------------|-------|-------------|
| NORMAL | 1000 | Normal closure |
| GOING_AWAY | 1001 | Endpoint going away |
| PROTOCOL_ERROR | 1002 | Protocol error |
| UNSUPPORTED_DATA | 1003 | Unsupported data type |
| NO_STATUS | 1005 | No status received |
| ABNORMAL | 1006 | Abnormal closure |
| INVALID_DATA | 1007 | Invalid frame payload |
| POLICY_VIOLATION | 1008 | Policy violation |
| MESSAGE_TOO_BIG | 1009 | Message too big |
| MANDATORY_EXTENSION | 1010 | Missing extension |
| INTERNAL_ERROR | 1011 | Internal server error |
| SERVICE_RESTART | 1012 | Service restart |
| TRY_AGAIN_LATER | 1013 | Try again later |
| TLS_HANDSHAKE | 1015 | TLS handshake failure |

## Connection States

| State | Value | Description |
|-------|-------|-------------|
| CONNECTING | 0 | Handshake in progress |
| OPEN | 1 | Connection established |
| CLOSING | 2 | Closing handshake initiated |
| CLOSED | 3 | Connection closed |

## Examples

### Real-Time Chat Application

```cursed
yeet "websocketz"

# Create chat server
sus server WebSocketConnection = ws_server_create(8080, "/chat")

# Create multiple chat rooms
sus general_room WebSocketRoom = ws_room_create("general", "General Discussion")
sus tech_room WebSocketRoom = ws_room_create("tech", "Tech Talk")

# Simulate clients joining
ws_room_join(&general_room, 1001)
ws_room_join(&general_room, 1002)
ws_room_join(&tech_room, 1001)    # Client can be in multiple rooms

# Broadcast messages
ws_room_broadcast(general_room, "User 1001: Hello everyone!")
ws_room_broadcast(tech_room, "User 1001: Anyone working on WebSocket projects?")

vibez.spill("Chat server running with " + 
           stringz.int_to_string(ws_room_get_connection_count(general_room)) + 
           " users in general")
```

### WebSocket with Compression

```cursed
yeet "websocketz"

# Negotiate compression extension
sus client_exts tea = "permessage-deflate; client_max_window_bits"
sus server_exts tea = "permessage-deflate"
sus negotiated tea = ws_extension_negotiate(client_exts, server_exts)

lowkey stringz.length(negotiated) > 0 {
    vibez.spill("Using compression: " + negotiated)
    
    # Compress message
    sus original tea = "This is a long message that will benefit from compression"
    sus compressed tea = ws_extension_compress(original, negotiated)
    
    # Send compressed data
    sus conn WebSocketConnection = ws_connection_create("ws://localhost:8080/compress", cap)
    ws_connection_open(&conn)
    ws_send_binary(&conn, compressed)
    
    # Decompress on receive
    sus decompressed tea = ws_extension_decompress(compressed, negotiated)
    vibez.spill("Decompressed: " + decompressed)
}
```

### Secure WebSocket with Origin Validation

```cursed
yeet "websocketz"

# Define allowed origins
sus allowed_origins [3]tea
allowed_origins[0] = "https://myapp.com"
allowed_origins[1] = "https://app.myapp.com"
allowed_origins[2] = "https://localhost:3000"

# Validate client origin
sus client_origin tea = "https://myapp.com"
lowkey ws_validate_origin(client_origin, allowed_origins, 3) {
    # Create secure connection
    sus conn WebSocketConnection = ws_connection_create("wss://secure.myapp.com/api", cap)
    ws_connection_open(&conn)
    
    # Set up content filtering
    sus blocked_words [3]tea
    blocked_words[0] = "spam"
    blocked_words[1] = "malicious"
    blocked_words[2] = "forbidden"
    
    # Filter incoming messages
    sus message tea = "This is a clean message"
    lowkey ws_content_filter(message, blocked_words, 3) {
        ws_send_text(&conn, message)
        vibez.spill("Message sent: " + message)
    } else {
        vibez.spill("Message blocked by content filter")
    }
} else {
    vibez.spill("Origin not allowed: " + client_origin)
}
```

### Multi-Room Broadcasting System

```cursed
yeet "websocketz"

# Create multiple themed rooms
sus rooms [5]WebSocketRoom
rooms[0] = ws_room_create("announcements", "System Announcements")
rooms[1] = ws_room_create("general", "General Chat")
rooms[2] = ws_room_create("tech", "Technical Discussion")
rooms[3] = ws_room_create("random", "Random Topics")
rooms[4] = ws_room_create("help", "Help & Support")

# Add users to different rooms
bestie i normie = 1001; i <= 1010; i++ {
    # Add all users to announcements
    ws_room_join(&rooms[0], i)
    
    # Distribute users across other rooms
    sus room_index normie = (i - 1001) % 4 + 1
    ws_room_join(&rooms[room_index], i)
}

# Broadcast system announcement to all users
sus announcement_count normie = ws_room_broadcast(rooms[0], "System maintenance scheduled for tonight")
vibez.spill("System announcement sent to " + stringz.int_to_string(announcement_count) + " users")

# Broadcast to specific rooms
ws_room_broadcast(rooms[2], "New programming tutorial available!")
ws_room_broadcast(rooms[4], "Support team is online and ready to help")

# Display room statistics
bestie i normie = 0; i < 5; i++ {
    sus count normie = ws_room_get_connection_count(rooms[i])
    vibez.spill("Room '" + rooms[i].name + "' has " + stringz.int_to_string(count) + " users")
}
```

### WebSocket Health Monitoring

```cursed
yeet "websocketz"

# Create connection with health monitoring
sus conn WebSocketConnection = ws_connection_create("ws://localhost:8080/monitor", cap)
conn.ping_interval = 10  # Ping every 10 seconds
ws_connection_open(&conn)

# Send periodic pings
bestie ping_count normie = 1; ping_count <= 5; ping_count++ {
    sus ping_payload tea = "health_check_" + stringz.int_to_string(ping_count)
    ws_send_ping(&conn, ping_payload)
    vibez.spill("Sent ping: " + ping_payload)
    
    # Simulate waiting for pong response
    vibez.spill("Waiting for pong response...")
    
    # In real implementation, would wait for actual pong
    ws_send_pong(&conn, ping_payload)
    vibez.spill("Received pong: " + ping_payload)
}

# Check connection health
lowkey ws_connection_is_open(conn) {
    vibez.spill("Connection is healthy")
} else {
    vibez.spill("Connection health check failed")
}
```

## Testing

Run the comprehensive test suite:

```bash
./zig-out/bin/cursed stdlib/websocketz/test_websocketz.csd
```

The test suite covers:
- Frame creation, serialization, and parsing
- Handshake request and response validation
- Connection lifecycle management
- Text and binary messaging
- Ping/pong health checks
- Room-based broadcasting
- Client and server APIs
- Extension negotiation
- Security features
- Integration scenarios
- Performance testing

## Performance Considerations

- **Frame Size**: Default maximum frame size is 1MB (configurable)
- **Message Queue**: Supports up to 100 queued messages per connection
- **Room Capacity**: Each room supports up to 50 connections by default
- **Ping Interval**: Default ping interval is 30 seconds (configurable)
- **Memory Usage**: Efficient frame processing with minimal memory allocation

## Security Features

- **Origin Validation**: Prevents cross-origin WebSocket abuse
- **Rate Limiting**: Configurable message rate limits per connection
- **Content Filtering**: Block messages containing unwanted content
- **Secure Handshake**: Proper key generation and validation
- **Connection Limits**: Configurable maximum connections per room

## Browser Compatibility

WebSocket is supported by all modern browsers:
- Chrome 4+
- Firefox 4+
- Safari 5+
- Edge 12+
- Opera 10.70+
- Internet Explorer 10+

## Migration from Existing WebSocket Libraries

1. **Replace imports**: Change existing WebSocket imports to `yeet "websocketz"`
2. **Update connection code**: Use `ws_client_connect()` for clients and `ws_server_create()` for servers
3. **Modify message handling**: Use `ws_send_text()`, `ws_send_binary()`, and `ws_receive_message()`
4. **Implement room management**: Use `ws_room_*` functions for broadcasting
5. **Add security**: Implement origin validation and content filtering
6. **Test thoroughly**: Validate handshake, messaging, and disconnection scenarios

## Implementation Notes

- **Pure CURSED**: No external dependencies, implemented entirely in CURSED
- **RFC 6455 Compliant**: Follows WebSocket specification requirements
- **Production Ready**: Comprehensive error handling and edge case coverage
- **Memory Efficient**: Optimized frame processing and message queuing
- **Cross-Platform**: Works on all platforms supporting CURSED
- **Thread Safe**: Designed for concurrent access with proper synchronization

The websocketz module provides a complete, production-ready WebSocket implementation that enables real-time, bidirectional communication between clients and servers with advanced features like room-based broadcasting, compression, and security.
