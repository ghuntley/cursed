yeet "glowup_http"

fr fr Example WebSocket implementation using glowup_http framework

fr fr Initialize the framework
glowup_http_main()

vibez.spill("WebSocket Examples")
vibez.spill("=================")

fr fr Test WebSocket handshake
vibez.spill("Testing WebSocket handshake...")
sus client_key tea = "dGhlIHNhbXBsZSBub25jZQ=="
sus accept_key tea = websocket_handshake(client_key)
vibez.spill("Client Key: " + client_key)
vibez.spill("Accept Key: " + accept_key)

fr fr Test WebSocket frame creation
vibez.spill("Testing WebSocket frame creation...")
sus text_frame WebSocketFrame = websocket_create_frame(1, "Hello WebSocket!")
sus binary_frame WebSocketFrame = websocket_create_frame(2, "Binary data here")
sus ping_frame WebSocketFrame = websocket_create_frame(9, "")
sus pong_frame WebSocketFrame = websocket_create_frame(10, "")

vibez.spill("Text Frame - Opcode: " + http_int_to_string(text_frame.opcode))
vibez.spill("Text Frame - Payload: " + text_frame.payload)
vibez.spill("Text Frame - FIN: " + (text_frame.fin ? "true" : "false"))
vibez.spill("Text Frame - Masked: " + (text_frame.masked ? "true" : "false"))

vibez.spill("Binary Frame - Opcode: " + http_int_to_string(binary_frame.opcode))
vibez.spill("Ping Frame - Opcode: " + http_int_to_string(ping_frame.opcode))
vibez.spill("Pong Frame - Opcode: " + http_int_to_string(pong_frame.opcode))

fr fr Test WebSocket message sending
vibez.spill("Testing WebSocket message sending...")
websocket_send_text("Hello from CURSED WebSocket!")
websocket_send_text("This is a test message")
websocket_send_text("{\"type\": \"message\", \"content\": \"JSON over WebSocket\"}")

websocket_send_binary("Binary message content")
websocket_send_binary("Another binary message")

fr fr Test WebSocket control frames
vibez.spill("Testing WebSocket control frames...")
websocket_ping()
websocket_pong()

fr fr Simulate WebSocket conversation
vibez.spill("Simulating WebSocket conversation...")
sus messages tea = "Welcome to the chat!"
websocket_send_text(messages)

sus user_message tea = "Hello everyone!"
websocket_send_text("User: " + user_message)

sus system_message tea = "System: User joined the chat"
websocket_send_text(system_message)

sus json_message tea = "{\"type\": \"notification\", \"message\": \"New user online\"}"
websocket_send_text(json_message)

fr fr Test different WebSocket opcodes
vibez.spill("Testing different WebSocket opcodes...")
sus continuation_frame WebSocketFrame = websocket_create_frame(0, "Continuation frame")
sus text_msg_frame WebSocketFrame = websocket_create_frame(1, "Text message frame")
sus binary_msg_frame WebSocketFrame = websocket_create_frame(2, "Binary message frame")
sus close_frame WebSocketFrame = websocket_create_frame(8, "Close frame")
sus ping_ctrl_frame WebSocketFrame = websocket_create_frame(9, "Ping control frame")
sus pong_ctrl_frame WebSocketFrame = websocket_create_frame(10, "Pong control frame")

vibez.spill("Continuation Frame (0): " + continuation_frame.payload)
vibez.spill("Text Message Frame (1): " + text_msg_frame.payload)
vibez.spill("Binary Message Frame (2): " + binary_msg_frame.payload)
vibez.spill("Close Frame (8): " + close_frame.payload)
vibez.spill("Ping Control Frame (9): " + ping_ctrl_frame.payload)
vibez.spill("Pong Control Frame (10): " + pong_ctrl_frame.payload)

fr fr Test WebSocket with different payload sizes
vibez.spill("Testing WebSocket with different payload sizes...")
sus small_payload tea = "Small"
sus medium_payload tea = "This is a medium sized WebSocket message payload"
sus large_payload tea = "This is a very large WebSocket message payload that contains much more content and data to test the frame handling capabilities of the WebSocket implementation"

websocket_send_text(small_payload)
websocket_send_text(medium_payload)
websocket_send_text(large_payload)

fr fr Test WebSocket heartbeat mechanism
vibez.spill("Testing WebSocket heartbeat mechanism...")
bestie i := 0; i < 5; i++ {
    websocket_ping()
    vibez.spill("Heartbeat ping " + http_int_to_string(i + 1))
    websocket_pong()
    vibez.spill("Heartbeat pong " + http_int_to_string(i + 1))
}

fr fr Test WebSocket error handling
vibez.spill("Testing WebSocket error handling...")
sus error_frame WebSocketFrame = websocket_create_frame(8, "Connection closed due to error")
vibez.spill("Error frame created with opcode: " + http_int_to_string(error_frame.opcode))

fr fr Test WebSocket constants
vibez.spill("Testing WebSocket constants...")
vibez.spill("WebSocket Magic String: " + WEBSOCKET_MAGIC)

fr fr Simulate WebSocket server responses
vibez.spill("Simulating WebSocket server responses...")
sus server_welcome tea = "Welcome to the WebSocket server!"
sus server_info tea = "Server: glowup_http WebSocket implementation"
sus server_stats tea = "{\"connections\": 42, \"messages\": 1337, \"uptime\": 86400}"

websocket_send_text(server_welcome)
websocket_send_text(server_info)
websocket_send_text(server_stats)

fr fr Test WebSocket client simulation
vibez.spill("Simulating WebSocket client messages...")
sus client_hello tea = "Hello server!"
sus client_data tea = "{\"action\": \"getData\", \"params\": {\"limit\": 10}}"
sus client_keepalive tea = "keepalive"

websocket_send_text(client_hello)
websocket_send_text(client_data)
websocket_send_text(client_keepalive)

vibez.spill("WebSocket examples completed successfully!")
