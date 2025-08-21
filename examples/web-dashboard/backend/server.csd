# CURSED Web Dashboard - Backend API Server
# RESTful API server with WebSocket support

yeet "networkz"
yeet "jsonz"
yeet "filez"
yeet "timez"
yeet "cryptz"
yeet "concurrenz"
yeet "../shared/models"
yeet "../shared/config"
yeet "../shared/database"

# Global server state
squad ServerState {
    sus active_connections drip
    sus total_requests drip
    sus start_time drip
    sus websocket_connections chan<WebSocketMessage>
    sus metrics_channel chan<SystemMetrics>
}

sus server_state ServerState = ServerState {
    active_connections: 0,
    total_requests: 0,
    start_time: timez.now(),
    websocket_connections: make_channel(),
    metrics_channel: make_channel()
}

# Authentication middleware
slay authenticate_request(headers JsonValue) yikes<User> {
    sus auth_header tea = headers["Authorization"].as_string() fam {
        yikes "Missing Authorization header"
    }
    
    ready (!auth_header.starts_with("Bearer ")) {
        yikes "Invalid Authorization header format"
    }
    
    sus token tea = auth_header.substring(7) # Remove "Bearer "
    
    sus session Session = load_session(token) fam {
        yikes "Invalid or expired session"
    }
    
    # Check if session is expired
    sus current_time drip = timez.now()
    ready (session.expires_at < current_time) {
        delete_session(token) fam { }
        yikes "Session expired"
    }
    
    sus user User = load_user(session.user_id) fam {
        yikes "User not found"
    }
    
    ready (!user.is_active) {
        yikes "User account is disabled"
    }
    
    damn user
}

# API Handlers

slay handle_login(request_body tea) tea {
    sus parsed JsonValue = jsonz.parse(request_body) fam {
        damn create_error_response("Invalid JSON in request body")
    }
    
    sus username tea = parsed["username"].as_string() fam {
        damn create_error_response("Missing username")
    }
    
    sus password tea = parsed["password"].as_string() fam {
        damn create_error_response("Missing password")
    }
    
    sus user User = find_user_by_username(username) fam {
        damn create_error_response("Invalid username or password")
    }
    
    # Verify password hash
    sus password_hash tea = cryptz.sha256(password + "salt")
    ready (user.password_hash != password_hash) {
        damn create_error_response("Invalid username or password")
    }
    
    # Create new session
    sus session Session = Session {
        token: generate_session_token(),
        user_id: user.id,
        created_at: timez.now(),
        expires_at: timez.now() + get_session_timeout(),
        ip_address: "127.0.0.1" # Would get from request in real implementation
    }
    
    save_session(session) fam {
        damn create_error_response("Failed to create session")
    }
    
    sus response_data tea = "{"
    response_data = response_data + "\"token\":\"" + session.token + "\""
    response_data = response_data + ",\"user\":" + user_to_json(user)
    response_data = response_data + "}"
    
    damn create_success_response("Login successful", response_data)
}

slay handle_register(request_body tea) tea {
    sus parsed JsonValue = jsonz.parse(request_body) fam {
        damn create_error_response("Invalid JSON in request body")
    }
    
    sus username tea = parsed["username"].as_string() fam {
        damn create_error_response("Missing username")
    }
    
    sus email tea = parsed["email"].as_string() fam {
        damn create_error_response("Missing email")
    }
    
    sus password tea = parsed["password"].as_string() fam {
        damn create_error_response("Missing password")
    }
    
    # Validate input
    ready (!validate_username(username)) {
        damn create_error_response("Invalid username")
    }
    
    ready (!validate_email(email)) {
        damn create_error_response("Invalid email")
    }
    
    # Check if user already exists
    sus existing_user User = find_user_by_username(username) fam {
        # User doesn't exist, which is good
    }
    otherwise {
        damn create_error_response("Username already taken")
    }
    
    # Create new user
    sus user User = User {
        id: generate_user_id(),
        username: username,
        email: email,
        password_hash: cryptz.sha256(password + "salt"),
        created_at: timez.now(),
        is_admin: nah,
        is_active: based
    }
    
    save_user(user) fam {
        damn create_error_response("Failed to create user")
    }
    
    damn create_success_response("User registered successfully", user_to_json(user))
}

slay handle_get_users(headers JsonValue) tea {
    sus user User = authenticate_request(headers) fam {
        damn create_error_response("Authentication required")
    }
    
    ready (!user.is_admin) {
        damn create_error_response("Admin privileges required")
    }
    
    sus users []User = list_all_users() fam {
        damn create_error_response("Failed to retrieve users")
    }
    
    sus users_json tea = "["
    bestie (sus i drip = 0; i < users.length(); i++) {
        ready (i > 0) { users_json = users_json + "," }
        users_json = users_json + user_to_json(users[i])
    }
    users_json = users_json + "]"
    
    damn create_success_response("Users retrieved successfully", users_json)
}

slay handle_get_metrics(headers JsonValue) tea {
    sus user User = authenticate_request(headers) fam {
        damn create_error_response("Authentication required")
    }
    
    sus metrics []SystemMetrics = get_recent_metrics(50) fam {
        damn create_error_response("Failed to retrieve metrics")
    }
    
    sus metrics_json tea = "["
    bestie (sus i drip = 0; i < metrics.length(); i++) {
        ready (i > 0) { metrics_json = metrics_json + "," }
        metrics_json = metrics_json + metrics_to_json(metrics[i])
    }
    metrics_json = metrics_json + "]"
    
    damn create_success_response("Metrics retrieved successfully", metrics_json)
}

slay handle_file_upload(headers JsonValue, file_data tea) tea {
    sus user User = authenticate_request(headers) fam {
        damn create_error_response("Authentication required")
    }
    
    # Validate file size
    ready (file_data.length() > get_max_file_size()) {
        damn create_error_response("File too large")
    }
    
    # Generate unique filename
    sus file_id drip = timez.now()
    sus filename tea = "upload_" + file_id.to_string() + ".bin"
    sus file_path tea = get_database_path() + "/files/" + filename
    
    filez.write_file(file_path, file_data) fam {
        damn create_error_response("Failed to save file")
    }
    
    # Create file record
    sus uploaded_file UploadedFile = UploadedFile {
        id: file_id,
        filename: filename,
        original_name: "uploaded_file",
        size: file_data.length(),
        mime_type: "application/octet-stream",
        uploaded_by: user.id,
        upload_time: timez.now()
    }
    
    sus file_json tea = uploaded_file_to_json(uploaded_file)
    
    damn create_success_response("File uploaded successfully", file_json)
}

slay handle_get_chat_messages(headers JsonValue) tea {
    sus user User = authenticate_request(headers) fam {
        damn create_error_response("Authentication required")
    }
    
    sus messages []ChatMessage = get_recent_messages(100) fam {
        damn create_error_response("Failed to retrieve messages")
    }
    
    sus messages_json tea = "["
    bestie (sus i drip = 0; i < messages.length(); i++) {
        ready (i > 0) { messages_json = messages_json + "," }
        messages_json = messages_json + chat_message_to_json(messages[i])
    }
    messages_json = messages_json + "]"
    
    damn create_success_response("Messages retrieved successfully", messages_json)
}

slay handle_post_chat_message(headers JsonValue, request_body tea) tea {
    sus user User = authenticate_request(headers) fam {
        damn create_error_response("Authentication required")
    }
    
    sus parsed JsonValue = jsonz.parse(request_body) fam {
        damn create_error_response("Invalid JSON in request body")
    }
    
    sus content tea = parsed["content"].as_string() fam {
        damn create_error_response("Missing message content")
    }
    
    sus message ChatMessage = ChatMessage {
        id: generate_message_id(),
        user_id: user.id,
        username: user.username,
        content: content,
        timestamp: timez.now(),
        message_type: "text"
    }
    
    save_chat_message(message) fam {
        damn create_error_response("Failed to save message")
    }
    
    # Broadcast message via WebSocket
    sus ws_message WebSocketMessage = WebSocketMessage {
        type: "chat",
        payload: chat_message_to_json(message),
        timestamp: timez.now()
    }
    
    go {
        server_state.websocket_connections <- ws_message
    }
    
    damn create_success_response("Message posted successfully", chat_message_to_json(message))
}

# HTTP Request Router
slay route_request(method tea, path tea, headers JsonValue, body tea) tea {
    server_state.total_requests = server_state.total_requests + 1
    
    # Add CORS headers
    sus response tea = ""
    
    ready (method == "OPTIONS") {
        damn create_cors_response()
    }
    
    ready (method == "POST" && path == "/api/auth/login") {
        response = handle_login(body)
    } otherwise ready (method == "POST" && path == "/api/auth/register") {
        response = handle_register(body)
    } otherwise ready (method == "GET" && path == "/api/users") {
        response = handle_get_users(headers)
    } otherwise ready (method == "GET" && path == "/api/metrics") {
        response = handle_get_metrics(headers)
    } otherwise ready (method == "POST" && path == "/api/upload") {
        response = handle_file_upload(headers, body)
    } otherwise ready (method == "GET" && path == "/api/chat/messages") {
        response = handle_get_chat_messages(headers)
    } otherwise ready (method == "POST" && path == "/api/chat/messages") {
        response = handle_post_chat_message(headers, body)
    } otherwise {
        response = create_error_response("Not Found")
    }
    
    damn add_cors_headers(response)
}

# WebSocket Handler
slay handle_websocket_connection(connection WebSocketConnection) {
    vibez.spill("New WebSocket connection established")
    server_state.active_connections = server_state.active_connections + 1
    
    go {
        # Listen for broadcast messages
        bestie (based) {
            sus message WebSocketMessage = <-server_state.websocket_connections
            
            # Send message to this connection
            # In a real implementation, this would send to the actual WebSocket
            vibez.spill("Broadcasting message: " + message.type)
        }
    }
    
    # Handle disconnection
    server_state.active_connections = server_state.active_connections - 1
    vibez.spill("WebSocket connection closed")
}

# Metrics Collection
slay collect_metrics() {
    bestie (based) {
        sus metrics SystemMetrics = SystemMetrics {
            timestamp: timez.now(),
            cpu_usage: get_cpu_usage(),
            memory_usage: get_memory_usage(),
            disk_usage: get_disk_usage(),
            active_connections: server_state.active_connections,
            requests_per_second: calculate_rps()
        }
        
        save_metrics(metrics) fam {
            vibez.spill("Failed to save metrics")
        }
        
        # Broadcast metrics via WebSocket
        sus ws_message WebSocketMessage = WebSocketMessage {
            type: "metrics",
            payload: metrics_to_json(metrics),
            timestamp: timez.now()
        }
        
        server_state.websocket_connections <- ws_message
        
        # Sleep for 30 seconds
        sleep(30000)
    }
}

# System monitoring functions (simplified implementations)
slay get_cpu_usage() drip {
    # In a real implementation, this would read from /proc/stat or similar
    damn (timez.now() % 100).to_int()  # Mock data
}

slay get_memory_usage() drip {
    # In a real implementation, this would read from /proc/meminfo
    damn ((timez.now() % 1000) + 200).to_int()  # Mock data
}

slay get_disk_usage() drip {
    # In a real implementation, this would use df or similar
    damn ((timez.now() % 500) + 100).to_int()  # Mock data
}

slay calculate_rps() drip {
    sus uptime drip = timez.now() - server_state.start_time
    ready (uptime == 0) { damn 0 }
    damn server_state.total_requests / uptime
}

# Utility functions
slay create_success_response(message tea, data tea) tea {
    sus response tea = "{"
    response = response + "\"success\":true"
    response = response + ",\"message\":\"" + message + "\""
    response = response + ",\"data\":" + data
    response = response + ",\"timestamp\":" + timez.now().to_string()
    response = response + "}"
    damn response
}

slay create_error_response(message tea) tea {
    sus response tea = "{"
    response = response + "\"success\":false"
    response = response + ",\"message\":\"" + message + "\""
    response = response + ",\"data\":null"
    response = response + ",\"timestamp\":" + timez.now().to_string()
    response = response + "}"
    damn response
}

slay create_cors_response() tea {
    damn "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type, Authorization\r\n\r\n"
}

slay add_cors_headers(response tea) tea {
    sus headers tea = "Access-Control-Allow-Origin: *\r\n"
    headers = headers + "Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\n"
    headers = headers + "Access-Control-Allow-Headers: Content-Type, Authorization\r\n"
    headers = headers + "Content-Type: application/json\r\n\r\n"
    damn "HTTP/1.1 200 OK\r\n" + headers + response
}

slay uploaded_file_to_json(file UploadedFile) tea {
    sus json_obj tea = "{"
    json_obj = json_obj + "\"id\":" + file.id.to_string()
    json_obj = json_obj + ",\"filename\":\"" + file.filename + "\""
    json_obj = json_obj + ",\"original_name\":\"" + file.original_name + "\""
    json_obj = json_obj + ",\"size\":" + file.size.to_string()
    json_obj = json_obj + ",\"mime_type\":\"" + file.mime_type + "\""
    json_obj = json_obj + ",\"uploaded_by\":" + file.uploaded_by.to_string()
    json_obj = json_obj + ",\"upload_time\":" + file.upload_time.to_string()
    json_obj = json_obj + "}"
    damn json_obj
}

# Main server function
slay main() yikes<void> {
    vibez.spill("Starting CURSED Web Dashboard Backend Server...")
    
    # Initialize configuration
    init_config("config/server.json") fam {
        vibez.spill("Using default configuration")
    }
    
    # Initialize database
    init_database(get_database_path()) fam {
        vibez.spill("Failed to initialize database: " + error_message)
        damn
    }
    
    # Create default admin user if none exists
    sus admin_user User = find_user_by_username("admin") fam {
        vibez.spill("Creating default admin user...")
        sus admin User = User {
            id: generate_user_id(),
            username: "admin",
            email: "admin@dashboard.local",
            password_hash: cryptz.sha256("admin123salt"),
            created_at: timez.now(),
            is_admin: based,
            is_active: based
        }
        
        save_user(admin) fam {
            vibez.spill("Failed to create admin user")
        }
    }
    
    # Start metrics collection in background
    go {
        collect_metrics()
    }
    
    # Start HTTP server
    sus port drip = get_server_port()
    vibez.spill("Server listening on port: " + port.to_string())
    vibez.spill("API endpoints available:")
    vibez.spill("  POST /api/auth/login")
    vibez.spill("  POST /api/auth/register")
    vibez.spill("  GET /api/users")
    vibez.spill("  GET /api/metrics")
    vibez.spill("  POST /api/upload")
    vibez.spill("  GET /api/chat/messages")
    vibez.spill("  POST /api/chat/messages")
    vibez.spill("  WS /ws")
    
    # In a real implementation, this would start the actual HTTP server
    # For now, we'll simulate the server running
    vibez.spill("Server started successfully!")
    vibez.spill("Visit http://localhost:" + port.to_string() + " to access the dashboard")
    
    # Keep server running
    bestie (based) {
        sleep(1000)
        
        # Clean up expired sessions periodically
        sus deleted_sessions drip = cleanup_expired_sessions() fam { damn 0 }
        ready (deleted_sessions > 0) {
            vibez.spill("Cleaned up " + deleted_sessions.to_string() + " expired sessions")
        }
    }
}

# Start the server
main() fam {
    vibez.spill("Server startup failed: " + error_message)
}
