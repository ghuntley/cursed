/// WebSocket Chat Example for CURSED
/// 
/// This example demonstrates real-time WebSocket communication
/// including message handling, connection management, and chat features.

import "stdlib::net::websocket";
import "stdlib::io";
import "stdlib::sync";

fn main() -> Result<(), Error> {
    println("💬 CURSED WebSocket Chat Example")?;
    println("================================")?;
    
    // Example 1: Basic WebSocket client setup
    basic_websocket_example()?;
    
    // Example 2: Chat client implementation
    chat_client_example()?;
    
    // Example 3: WebSocket server simulation
    websocket_server_example()?;
    
    // Example 4: Advanced WebSocket features
    advanced_websocket_features()?;
    
    println("✅ WebSocket examples completed!")?;
    Ok(())
}

fn basic_websocket_example() -> Result<(), Error> {
    println("\n🔌 Basic WebSocket Setup")?;
    println("------------------------")?;
    
    // WebSocket configuration
    sus config = WebSocketConfig::new()
        .max_message_size(1024 * 1024)  // 1MB max message
        .max_frame_size(64 * 1024)      // 64KB max frame
        .ping_interval(Some(Duration::from_secs(30)))
        .pong_timeout(Duration::from_secs(10))
        .auto_pong(true);
    
    println("WebSocket configuration:")?;
    println("  Max message size: {} bytes", config.max_message_size)?;
    println("  Max frame size: {} bytes", config.max_frame_size)?;
    println("  Ping interval: {:?}", config.ping_interval)?;
    println("  Pong timeout: {:?}", config.pong_timeout)?;
    println("  Auto pong: {}", config.auto_pong)?;
    
    // Create WebSocket client (simulation)
    println("Creating WebSocket client...")?;
    println("  URL: ws://localhost:8080/chat")?;
    println("  Protocol: WebSocket (RFC 6455)")?;
    println("  Extensions: per-message-deflate")?;
    
    // Connection states
    sus states = [
        ConnectionState::Connecting,
        ConnectionState::Open,
        ConnectionState::Closing,
        ConnectionState::Closed
    ];
    
    println("WebSocket connection states:")?;
    for state in states {
        println("  {:?}: Connection lifecycle state", state)?;
    }
    
    println("✓ Basic WebSocket setup demonstrated")?;
    
    Ok(())
}

fn chat_client_example() -> Result<(), Error> {
    println("\n💬 Chat Client Implementation")?;
    println("-----------------------------")?;
    
    // Simulate chat client setup
    println("Setting up chat client...")?;
    
    // User information
    sus username = "alice";
    sus user_id = "user_123";
    sus chat_room = "general";
    
    println("User profile:")?;
    println("  Username: {}", username)?;
    println("  User ID: {}", user_id)?;
    println("  Chat room: {}", chat_room)?;
    
    // Message types for chat
    demonstrate_chat_messages()?;
    
    // Connection establishment simulation
    println("Establishing WebSocket connection...")?;
    println("  1. HTTP Upgrade request sent")?;
    println("  2. Server responds with 101 Switching Protocols")?;
    println("  3. WebSocket handshake completed")?;
    println("  4. Connection state: Open")?;
    
    // Authentication message
    sus auth_message = create_chat_message("auth", json!({
        "type": "authenticate",
        "user_id": user_id,
        "username": username,
        "token": "jwt_token_here"
    }));
    
    println("Sending authentication message:")?;
    println("  Type: authenticate")?;
    println("  User: {}", username)?;
    println("  Message size: {} bytes", auth_message.len())?;
    
    // Join room message
    sus join_message = create_chat_message("join_room", json!({
        "type": "join_room",
        "room": chat_room,
        "user_id": user_id
    }));
    
    println("Joining chat room:")?;
    println("  Room: {}", chat_room)?;
    println("  Message size: {} bytes", join_message.len())?;
    
    // Chat message examples
    simulate_chat_conversation(username)?;
    
    // Heartbeat and connection management
    println("Connection management:")?;
    println("  Sending ping frames every 30 seconds")?;
    println("  Expecting pong responses within 10 seconds")?;
    println("  Auto-reconnect on connection loss")?;
    
    Ok(())
}

fn demonstrate_chat_messages() -> Result<(), Error> {
    println("Chat message types:")?;
    
    // Text chat message
    sus text_msg = WebSocketMessage::text(json!({
        "type": "chat_message",
        "user": "alice",
        "room": "general", 
        "message": "Hello everyone!",
        "timestamp": "2024-01-15T10:30:00Z"
    }));
    
    println("  Text message: {} bytes", text_msg.len())?;
    
    // User joined notification
    sus join_notification = WebSocketMessage::text(json!({
        "type": "user_joined",
        "user": "bob",
        "room": "general",
        "timestamp": "2024-01-15T10:31:00Z"
    }));
    
    println("  Join notification: {} bytes", join_notification.len())?;
    
    // Private message
    sus private_msg = WebSocketMessage::text(json!({
        "type": "private_message",
        "from": "alice",
        "to": "bob",
        "message": "Hey Bob, how are you?",
        "timestamp": "2024-01-15T10:32:00Z"
    }));
    
    println("  Private message: {} bytes", private_msg.len())?;
    
    // Typing indicator
    sus typing_msg = WebSocketMessage::text(json!({
        "type": "typing",
        "user": "alice",
        "room": "general",
        "typing": true
    }));
    
    println("  Typing indicator: {} bytes", typing_msg.len())?;
    
    // File sharing message (metadata)
    sus file_msg = WebSocketMessage::text(json!({
        "type": "file_share",
        "user": "alice",
        "room": "general",
        "file_name": "document.pdf",
        "file_size": 1024000,
        "file_url": "https://cdn.example.com/files/abc123.pdf",
        "timestamp": "2024-01-15T10:33:00Z"
    }));
    
    println("  File share: {} bytes", file_msg.len())?;
    
    // Emoji reaction
    sus reaction_msg = WebSocketMessage::text(json!({
        "type": "reaction",
        "user": "bob",
        "room": "general",
        "message_id": "msg_456",
        "emoji": "👍",
        "action": "add"
    }));
    
    println("  Emoji reaction: {} bytes", reaction_msg.len())?;
    
    Ok(())
}

fn simulate_chat_conversation() -> Result<(), Error> {
    println("Simulating chat conversation:")?;
    
    // Conversation messages
    sus messages = [
        ("alice", "Hello everyone! 👋"),
        ("bob", "Hey Alice! How's your day going?"),
        ("charlie", "Good morning! Just joined the channel"),
        ("alice", "Going well, thanks! Working on some CURSED code"),
        ("bob", "Nice! CURSED has great networking features"),
        ("charlie", "I should learn CURSED, heard it's awesome"),
        ("alice", "You definitely should! The WebSocket support is fantastic"),
        ("bob", "Agreed! Real-time communication is so smooth"),
    ];
    
    for (i, (user, message)) in messages.iter().enumerate() {
        sus timestamp = format!("2024-01-15T10:{}:00Z", 30 + i);
        sus chat_message = json!({
            "type": "chat_message",
            "user": user,
            "room": "general",
            "message": message,
            "timestamp": timestamp,
            "message_id": format!("msg_{}", 100 + i)
        });
        
        println("  [{}] {}: {}", timestamp[11..16].to_string(), user, message)?;
        
        // Simulate message processing delay
        if i % 3 == 0 {
            println("    → Message delivered and acknowledged")?;
        }
    }
    
    // Simulate real-time features
    println("Real-time features:")?;
    println("  ✓ Message delivery confirmations")?;
    println("  ✓ Typing indicators")?;
    println("  ✓ Online/offline status")?;
    println("  ✓ Message read receipts")?;
    println("  ✓ Emoji reactions")?;
    
    Ok(())
}

fn websocket_server_example() -> Result<(), Error> {
    println("\n🖥️ WebSocket Server Simulation")?;
    println("------------------------------")?;
    
    // Server configuration
    println("WebSocket server configuration:")?;
    println("  Listen address: 0.0.0.0:8080")?;
    println("  Max connections: 1000")?;
    println("  Max message size: 1MB")?;
    println("  Heartbeat interval: 30s")?;
    println("  Room capacity: 100 users per room")?;
    
    // Server states
    println("Server lifecycle:")?;
    println("  1. Initialize server socket")?;
    println("  2. Bind to address and port")?;
    println("  3. Start listening for connections")?;
    println("  4. Accept WebSocket handshakes")?;
    println("  5. Handle client messages")?;
    println("  6. Broadcast to room members")?;
    
    // Connection handling
    println("Connection handling:")?;
    println("  ✓ HTTP upgrade validation")?;
    println("  ✓ WebSocket key verification")?;
    println("  ✓ Protocol negotiation")?;
    println("  ✓ Extension support (compression)")?;
    println("  ✓ Client authentication")?;
    
    // Message routing
    println("Message routing:")?;
    println("  • Chat messages → Room broadcast")?;
    println("  • Private messages → Direct delivery")?;
    println("  • System messages → All clients")?;
    println("  • Typing indicators → Room members")?;
    println("  • Presence updates → Subscribed clients")?;
    
    // Room management
    println("Room management:")?;
    sus rooms = ["general", "random", "dev-team", "announcements"];
    for room in rooms {
        sus user_count = match room {
            "general" => 45,
            "random" => 23,
            "dev-team" => 12,
            "announcements" => 78,
            _ => 0,
        };
        println("  Room '{}': {} active users", room, user_count)?;
    }
    
    // Server statistics
    println("Server statistics:")?;
    println("  Active connections: 158")?;
    println("  Messages per second: 24")?;
    println("  Average message size: 142 bytes")?;
    println("  Total rooms: {}", rooms.len())?;
    println("  Uptime: 2 hours 34 minutes")?;
    
    Ok(())
}

fn advanced_websocket_features() -> Result<(), Error> {
    println("\n🚀 Advanced WebSocket Features")?;
    println("------------------------------")?;
    
    // Compression
    println("Message compression:")?;
    sus compression_config = CompressionConfig {
        enable_per_message_deflate: true,
        deflate_no_context_takeover: false,
        deflate_max_window_bits: 15,
    };
    
    println("  Per-message deflate: {}", compression_config.enable_per_message_deflate)?;
    println("  No context takeover: {}", compression_config.deflate_no_context_takeover)?;
    println("  Max window bits: {}", compression_config.deflate_max_window_bits)?;
    
    // Large message handling
    println("Large message handling:")?;
    sus large_data = vec![0u8; 100 * 1024]; // 100KB
    sus binary_message = WebSocketMessage::binary(large_data);
    println("  Large binary message: {} bytes", binary_message.len())?;
    println("  Fragmentation: Automatic for messages > 64KB")?;
    println("  Streaming: Supported for real-time data")?;
    
    // Connection resilience
    println("Connection resilience:")?;
    println("  ✓ Automatic reconnection with exponential backoff")?;
    println("  ✓ Message queuing during disconnection")?;
    println("  ✓ Duplicate message detection")?;
    println("  ✓ Connection health monitoring")?;
    println("  ✓ Graceful degradation")?;
    
    // Performance features
    println("Performance optimizations:")?;
    println("  • Frame batching for multiple small messages")?;
    println("  • Connection pooling for multiple endpoints")?;
    println("  • Message prioritization (control > data)")?;
    println("  • Backpressure handling")?;
    println("  • Memory-efficient frame processing")?;
    
    // Security features
    println("Security features:")?;
    println("  🔒 TLS/SSL encryption support")?;
    println("  🔒 Origin validation")?;
    println("  🔒 Authentication token verification")?;
    println("  🔒 Rate limiting per connection")?;
    println("  🔒 Message size limits")?;
    println("  🔒 Connection timeout protection")?;
    
    // Close code handling
    println("Close codes and reasons:")?;
    sus close_codes = [
        (CloseCode::NORMAL, "Normal closure"),
        (CloseCode::GOING_AWAY, "Server restart"),
        (CloseCode::PROTOCOL_ERROR, "Invalid frame"),
        (CloseCode::UNSUPPORTED_DATA, "Wrong data type"),
        (CloseCode::MESSAGE_TOO_BIG, "Message exceeded limit"),
        (CloseCode::INTERNAL_ERROR, "Server error")
    ];
    
    for (code, description) in close_codes {
        println("  {}: {}", code, description)?;
    }
    
    // Extension support
    println("WebSocket extensions:")?;
    println("  📦 per-message-deflate (compression)")?;
    println("  📦 x-webkit-deflate-frame (legacy)")?;
    println("  📦 permessage-deflate (standard)")?;
    
    Ok(())
}

fn create_chat_message(msg_type: &str, data: String) -> WebSocketMessage {
    WebSocketMessage::text(data)
}

// Helper function to simulate JSON creation
fn json!(data: any) -> String {
    // Simplified JSON simulation for demo
    match data {
        _ => format!("{{\"demo\": \"json\", \"type\": \"simulated\"}}")
    }
}
