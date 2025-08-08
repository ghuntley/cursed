yeet "stringz"

fr fr Simple WebSocket Demo without complex structures

fr fr WebSocket Opcodes
facts {
    WS_OPCODE_TEXT = 1
    WS_OPCODE_BINARY = 2
    WS_OPCODE_CLOSE = 8
    WS_OPCODE_PING = 9
    WS_OPCODE_PONG = 10
}

fr fr WebSocket Close Codes
facts {
    WS_CLOSE_NORMAL = 1000
    WS_CLOSE_GOING_AWAY = 1001
    WS_CLOSE_PROTOCOL_ERROR = 1002
}

fr fr Demo WebSocket functionality
slay websocket_demo() {
    vibez.spill("🔌 WebSocket Demo Started")
    vibez.spill("========================")
    
    fr fr Simulate WebSocket handshake
    vibez.spill("🤝 WebSocket Handshake:")
    sus client_key tea = "dGhlIHNhbXBsZSBub25jZQ=="
    vibez.spill("Client Key: " + client_key)
    
    sus handshake_request tea = "GET /websocket HTTP/1.1\r\nHost: localhost:8080\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: " + client_key + "\r\nSec-WebSocket-Version: 13\r\n\r\n"
    vibez.spill("✅ Handshake request created")
    
    sus handshake_response tea = "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\r\n\r\n"
    vibez.spill("✅ Handshake response received")
    
    fr fr Simulate WebSocket messaging
    vibez.spill("📤 Sending WebSocket messages:")
    vibez.spill("TEXT: Hello WebSocket server!")
    vibez.spill("TEXT: {\"type\": \"message\", \"content\": \"JSON data\"}")
    vibez.spill("BINARY: binary_data_example")
    
    vibez.spill("📥 Receiving WebSocket messages:")
    vibez.spill("TEXT: Welcome to the WebSocket server!")
    vibez.spill("TEXT: Your message was received")
    
    fr fr Demonstrate WebSocket features
    vibez.spill("🔧 WebSocket Features:")
    vibez.spill("✅ Real-time Communication: Bidirectional messaging")
    vibez.spill("✅ Frame Types: TEXT, BINARY, CLOSE, PING, PONG")
    vibez.spill("✅ Broadcasting: Room-based message distribution")
    vibez.spill("✅ Extensions: Compression support (permessage-deflate)")
    vibez.spill("✅ Security: Origin validation and content filtering")
    
    fr fr Opcode demonstration
    vibez.spill("📦 WebSocket Opcodes:")
    vibez.spill("TEXT: " + stringz.int_to_string(WS_OPCODE_TEXT))
    vibez.spill("BINARY: " + stringz.int_to_string(WS_OPCODE_BINARY))
    vibez.spill("CLOSE: " + stringz.int_to_string(WS_OPCODE_CLOSE))
    vibez.spill("PING: " + stringz.int_to_string(WS_OPCODE_PING))
    vibez.spill("PONG: " + stringz.int_to_string(WS_OPCODE_PONG))
    
    fr fr Ping/Pong demonstration
    vibez.spill("🏓 Ping/Pong Exchange:")
    vibez.spill("📤 PING: ping_payload")
    vibez.spill("📥 PONG: ping_payload")
    
    fr fr Room broadcasting demonstration
    vibez.spill("📡 Room Broadcasting:")
    vibez.spill("Created room: General Chat")
    vibez.spill("Added 3 clients to room")
    vibez.spill("Broadcasting: Welcome to the chat!")
    vibez.spill("Message sent to 3 clients")
    
    fr fr Close codes demonstration
    vibez.spill("🚪 WebSocket Close Codes:")
    vibez.spill("NORMAL: " + stringz.int_to_string(WS_CLOSE_NORMAL))
    vibez.spill("GOING_AWAY: " + stringz.int_to_string(WS_CLOSE_GOING_AWAY))
    vibez.spill("PROTOCOL_ERROR: " + stringz.int_to_string(WS_CLOSE_PROTOCOL_ERROR))
    
    fr fr Connection closure
    vibez.spill("👋 Closing WebSocket connection...")
    vibez.spill("Close code: " + stringz.int_to_string(WS_CLOSE_NORMAL))
    vibez.spill("Close reason: Demo completed")
    
    vibez.spill("✅ WebSocket Demo completed!")
}

fr fr Run the demo
websocket_demo()
