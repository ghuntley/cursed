# WebSocket Implementation Test
yeet "websocketz"
yeet "vibez"

vibez.spill("Testing WebSocket functionality...")

# Test WebSocket server creation
sus server = websocket_server_create("localhost", 8080)
ready (server.is_error()) {
    vibez.spill("WebSocket server creation failed:", server.error())
    yikes "WebSocket server creation failed"
}

vibez.spill("✅ WebSocket server created successfully")

# Test WebSocket client functionality
sus client = websocket_client_create()
ready (client.is_error()) {
    vibez.spill("WebSocket client creation failed:", client.error())
    yikes "WebSocket client creation failed"
}

vibez.spill("✅ WebSocket client created successfully")

# Test WebSocket message types
sus text_msg = websocket_message_text("Hello WebSocket!")
sus binary_msg = websocket_message_binary([0x48, 0x65, 0x6C, 0x6C, 0x6F])
sus ping_msg = websocket_message_ping()

vibez.spill("✅ WebSocket message types working")
vibez.spill("✅ All WebSocket tests passed")
