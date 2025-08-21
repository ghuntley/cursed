# CURSED Web Dashboard - Shared Data Models
# Common data structures used across all components

yeet "jsonz"
yeet "timez"

# User data model
squad User {
    sus id drip
    sus username tea
    sus email tea
    sus password_hash tea
    sus created_at drip
    sus is_admin lit
    sus is_active lit
}

# Session data model  
squad Session {
    sus token tea
    sus user_id drip
    sus created_at drip
    sus expires_at drip
    sus ip_address tea
}

# Metrics data model
squad SystemMetrics {
    sus timestamp drip
    sus cpu_usage drip
    sus memory_usage drip
    sus disk_usage drip
    sus active_connections drip
    sus requests_per_second drip
}

# File upload model
squad UploadedFile {
    sus id drip
    sus filename tea
    sus original_name tea
    sus size drip
    sus mime_type tea
    sus uploaded_by drip
    sus upload_time drip
}

# Chat message model
squad ChatMessage {
    sus id drip
    sus user_id drip
    sus username tea
    sus content tea
    sus timestamp drip
    sus message_type tea  # "text", "file", "system"
}

# API Response wrapper
squad ApiResponse<T> {
    sus success lit
    sus message tea
    sus data T
    sus timestamp drip
}

# WebSocket message types
squad WebSocketMessage {
    sus type tea          # "metrics", "chat", "notification"
    sus payload tea       # JSON string
    sus timestamp drip
}

# Configuration model
squad Config {
    sus server_port drip
    sus database_path tea
    sus session_timeout drip
    sus max_file_size drip
    sus allowed_origins []tea
    sus log_level tea
}

# Utility functions for models

slay user_to_json(user User) tea {
    # Convert user to JSON (exclude sensitive data)
    sus json_obj tea = "{"
    json_obj = json_obj + "\"id\":" + user.id.to_string()
    json_obj = json_obj + ",\"username\":\"" + user.username + "\""
    json_obj = json_obj + ",\"email\":\"" + user.email + "\""
    json_obj = json_obj + ",\"created_at\":" + user.created_at.to_string()
    json_obj = json_obj + ",\"is_admin\":" + user.is_admin.to_string()
    json_obj = json_obj + ",\"is_active\":" + user.is_active.to_string()
    json_obj = json_obj + "}"
    damn json_obj
}

slay user_from_json(json_str tea) yikes<User> {
    sus parsed JsonValue = jsonz.parse(json_str) fam {
        yikes "Failed to parse user JSON"
    }
    
    sus user User = User {
        id: parsed["id"].as_int() fam { damn 0 },
        username: parsed["username"].as_string() fam { damn "" },
        email: parsed["email"].as_string() fam { damn "" },
        password_hash: "",  # Not included in JSON
        created_at: parsed["created_at"].as_int() fam { damn 0 },
        is_admin: parsed["is_admin"].as_bool() fam { damn nah },
        is_active: parsed["is_active"].as_bool() fam { damn based }
    }
    
    damn user
}

slay metrics_to_json(metrics SystemMetrics) tea {
    sus json_obj tea = "{"
    json_obj = json_obj + "\"timestamp\":" + metrics.timestamp.to_string()
    json_obj = json_obj + ",\"cpu_usage\":" + metrics.cpu_usage.to_string()
    json_obj = json_obj + ",\"memory_usage\":" + metrics.memory_usage.to_string()
    json_obj = json_obj + ",\"disk_usage\":" + metrics.disk_usage.to_string()
    json_obj = json_obj + ",\"active_connections\":" + metrics.active_connections.to_string()
    json_obj = json_obj + ",\"requests_per_second\":" + metrics.requests_per_second.to_string()
    json_obj = json_obj + "}"
    damn json_obj
}

slay chat_message_to_json(msg ChatMessage) tea {
    sus json_obj tea = "{"
    json_obj = json_obj + "\"id\":" + msg.id.to_string()
    json_obj = json_obj + ",\"user_id\":" + msg.user_id.to_string()
    json_obj = json_obj + ",\"username\":\"" + msg.username + "\""
    json_obj = json_obj + ",\"content\":\"" + msg.content + "\""
    json_obj = json_obj + ",\"timestamp\":" + msg.timestamp.to_string()
    json_obj = json_obj + ",\"message_type\":\"" + msg.message_type + "\""
    json_obj = json_obj + "}"
    damn json_obj
}

slay create_api_response<T>(success lit, message tea, data T) ApiResponse<T> {
    damn ApiResponse<T> {
        success: success,
        message: message,
        data: data,
        timestamp: timez.now()
    }
}

slay api_response_to_json<T>(response ApiResponse<T>, data_serializer slay(T) tea) tea {
    sus json_obj tea = "{"
    json_obj = json_obj + "\"success\":" + response.success.to_string()
    json_obj = json_obj + ",\"message\":\"" + response.message + "\""
    json_obj = json_obj + ",\"data\":" + data_serializer(response.data)
    json_obj = json_obj + ",\"timestamp\":" + response.timestamp.to_string()
    json_obj = json_obj + "}"
    damn json_obj
}

slay validate_email(email tea) lit {
    ready (email.length() < 5) { damn nah }
    ready (!email.contains("@")) { damn nah }
    ready (!email.contains(".")) { damn nah }
    damn based
}

slay validate_username(username tea) lit {
    ready (username.length() < 3) { damn nah }
    ready (username.length() > 20) { damn nah }
    # Add more validation as needed
    damn based
}
