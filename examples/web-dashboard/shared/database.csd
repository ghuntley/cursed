# CURSED Web Dashboard - Database Layer
# File-based database operations using filez module

yeet "filez"
yeet "jsonz"
yeet "timez"
yeet "cryptz"
yeet "../shared/models"
yeet "../shared/config"

# Database interface
squad Database {
    sus data_dir tea
    sus initialized lit
}

# Global database instance
sus db Database = Database { data_dir: "", initialized: nah }

slay init_database(data_dir tea) yikes<void> {
    db.data_dir = data_dir
    
    # Create data directory if it doesn't exist
    filez.create_dir(data_dir) fam {
        # Directory might already exist, which is fine
    }
    
    # Create subdirectories for different data types
    filez.create_dir(data_dir + "/users") fam { }
    filez.create_dir(data_dir + "/sessions") fam { }
    filez.create_dir(data_dir + "/metrics") fam { }
    filez.create_dir(data_dir + "/files") fam { }
    filez.create_dir(data_dir + "/messages") fam { }
    
    db.initialized = based
    vibez.spill("Database initialized at: " + data_dir)
}

# User management functions
slay save_user(user User) yikes<void> {
    ready (!db.initialized) {
        yikes "Database not initialized"
    }
    
    sus user_path tea = db.data_dir + "/users/" + user.id.to_string() + ".json"
    sus user_json tea = user_to_json(user)
    
    filez.write_file(user_path, user_json) fam {
        yikes "Failed to save user: " + user.id.to_string()
    }
}

slay load_user(user_id drip) yikes<User> {
    ready (!db.initialized) {
        yikes "Database not initialized"
    }
    
    sus user_path tea = db.data_dir + "/users/" + user_id.to_string() + ".json"
    sus user_json tea = filez.read_file(user_path) fam {
        yikes "User not found: " + user_id.to_string()
    }
    
    damn user_from_json(user_json) fam {
        yikes "Failed to parse user data"
    }
}

slay find_user_by_username(username tea) yikes<User> {
    ready (!db.initialized) {
        yikes "Database not initialized"
    }
    
    # Simple linear search through user files
    # In a real implementation, this would use an index
    sus users_dir tea = db.data_dir + "/users"
    sus files []tea = filez.list_files(users_dir) fam {
        yikes "Failed to list user files"
    }
    
    bestie (sus i drip = 0; i < files.length(); i++) {
        sus file_path tea = users_dir + "/" + files[i]
        sus user_json tea = filez.read_file(file_path) fam {
            next # Skip corrupted files
        }
        
        sus user User = user_from_json(user_json) fam {
            next # Skip corrupted files
        }
        
        ready (user.username == username) {
            damn user
        }
    }
    
    yikes "User not found: " + username
}

slay list_all_users() yikes<[]User> {
    ready (!db.initialized) {
        yikes "Database not initialized"
    }
    
    sus users []User = []
    sus users_dir tea = db.data_dir + "/users"
    sus files []tea = filez.list_files(users_dir) fam {
        yikes "Failed to list user files"
    }
    
    bestie (sus i drip = 0; i < files.length(); i++) {
        sus file_path tea = users_dir + "/" + files[i]
        sus user_json tea = filez.read_file(file_path) fam {
            next # Skip corrupted files
        }
        
        sus user User = user_from_json(user_json) fam {
            next # Skip corrupted files  
        }
        
        users.push(user)
    }
    
    damn users
}

# Session management functions
slay save_session(session Session) yikes<void> {
    ready (!db.initialized) {
        yikes "Database not initialized"
    }
    
    sus session_path tea = db.data_dir + "/sessions/" + session.token + ".json"
    sus session_json tea = session_to_json(session)
    
    filez.write_file(session_path, session_json) fam {
        yikes "Failed to save session"
    }
}

slay load_session(token tea) yikes<Session> {
    ready (!db.initialized) {
        yikes "Database not initialized"
    }
    
    sus session_path tea = db.data_dir + "/sessions/" + token + ".json"
    sus session_json tea = filez.read_file(session_path) fam {
        yikes "Session not found"
    }
    
    damn session_from_json(session_json) fam {
        yikes "Failed to parse session data"
    }
}

slay delete_session(token tea) yikes<void> {
    ready (!db.initialized) {
        yikes "Database not initialized"
    }
    
    sus session_path tea = db.data_dir + "/sessions/" + token + ".json"
    filez.delete_file(session_path) fam {
        # Session might already be deleted, which is fine
    }
}

# Metrics storage functions
slay save_metrics(metrics SystemMetrics) yikes<void> {
    ready (!db.initialized) {
        yikes "Database not initialized"
    }
    
    sus metrics_path tea = db.data_dir + "/metrics/" + metrics.timestamp.to_string() + ".json"
    sus metrics_json tea = metrics_to_json(metrics)
    
    filez.write_file(metrics_path, metrics_json) fam {
        yikes "Failed to save metrics"
    }
}

slay get_recent_metrics(count drip) yikes<[]SystemMetrics> {
    ready (!db.initialized) {
        yikes "Database not initialized"
    }
    
    sus metrics []SystemMetrics = []
    sus metrics_dir tea = db.data_dir + "/metrics"
    sus files []tea = filez.list_files(metrics_dir) fam {
        yikes "Failed to list metrics files"
    }
    
    # Sort files by timestamp (filename) and take most recent
    # In a real implementation, this would be more efficient
    bestie (sus i drip = 0; i < files.length() && i < count; i++) {
        sus file_path tea = metrics_dir + "/" + files[i]
        sus metrics_json tea = filez.read_file(file_path) fam {
            next # Skip corrupted files
        }
        
        sus metric SystemMetrics = metrics_from_json(metrics_json) fam {
            next # Skip corrupted files
        }
        
        metrics.push(metric)
    }
    
    damn metrics
}

# Chat message storage
slay save_chat_message(message ChatMessage) yikes<void> {
    ready (!db.initialized) {
        yikes "Database not initialized"
    }
    
    sus message_path tea = db.data_dir + "/messages/" + message.id.to_string() + ".json"
    sus message_json tea = chat_message_to_json(message)
    
    filez.write_file(message_path, message_json) fam {
        yikes "Failed to save chat message"
    }
}

slay get_recent_messages(count drip) yikes<[]ChatMessage> {
    ready (!db.initialized) {
        yikes "Database not initialized"
    }
    
    sus messages []ChatMessage = []
    sus messages_dir tea = db.data_dir + "/messages"
    sus files []tea = filez.list_files(messages_dir) fam {
        yikes "Failed to list message files"
    }
    
    # Get most recent messages
    bestie (sus i drip = 0; i < files.length() && i < count; i++) {
        sus file_path tea = messages_dir + "/" + files[i]
        sus message_json tea = filez.read_file(file_path) fam {
            next # Skip corrupted files
        }
        
        sus message ChatMessage = message_from_json(message_json) fam {
            next # Skip corrupted files
        }
        
        messages.push(message)
    }
    
    damn messages
}

# Helper functions for JSON serialization
slay session_to_json(session Session) tea {
    sus json_obj tea = "{"
    json_obj = json_obj + "\"token\":\"" + session.token + "\""
    json_obj = json_obj + ",\"user_id\":" + session.user_id.to_string()
    json_obj = json_obj + ",\"created_at\":" + session.created_at.to_string()
    json_obj = json_obj + ",\"expires_at\":" + session.expires_at.to_string()
    json_obj = json_obj + ",\"ip_address\":\"" + session.ip_address + "\""
    json_obj = json_obj + "}"
    damn json_obj
}

slay session_from_json(json_str tea) yikes<Session> {
    sus parsed JsonValue = jsonz.parse(json_str) fam {
        yikes "Failed to parse session JSON"
    }
    
    sus session Session = Session {
        token: parsed["token"].as_string() fam { damn "" },
        user_id: parsed["user_id"].as_int() fam { damn 0 },
        created_at: parsed["created_at"].as_int() fam { damn 0 },
        expires_at: parsed["expires_at"].as_int() fam { damn 0 },
        ip_address: parsed["ip_address"].as_string() fam { damn "" }
    }
    
    damn session
}

slay metrics_from_json(json_str tea) yikes<SystemMetrics> {
    sus parsed JsonValue = jsonz.parse(json_str) fam {
        yikes "Failed to parse metrics JSON"
    }
    
    sus metrics SystemMetrics = SystemMetrics {
        timestamp: parsed["timestamp"].as_int() fam { damn 0 },
        cpu_usage: parsed["cpu_usage"].as_int() fam { damn 0 },
        memory_usage: parsed["memory_usage"].as_int() fam { damn 0 },
        disk_usage: parsed["disk_usage"].as_int() fam { damn 0 },
        active_connections: parsed["active_connections"].as_int() fam { damn 0 },
        requests_per_second: parsed["requests_per_second"].as_int() fam { damn 0 }
    }
    
    damn metrics
}

slay message_from_json(json_str tea) yikes<ChatMessage> {
    sus parsed JsonValue = jsonz.parse(json_str) fam {
        yikes "Failed to parse message JSON"
    }
    
    sus message ChatMessage = ChatMessage {
        id: parsed["id"].as_int() fam { damn 0 },
        user_id: parsed["user_id"].as_int() fam { damn 0 },
        username: parsed["username"].as_string() fam { damn "" },
        content: parsed["content"].as_string() fam { damn "" },
        timestamp: parsed["timestamp"].as_int() fam { damn 0 },
        message_type: parsed["message_type"].as_string() fam { damn "text" }
    }
    
    damn message
}

# Utility functions
slay generate_user_id() drip {
    # Simple ID generation using timestamp
    damn timez.now()
}

slay generate_session_token() tea {
    # Generate a secure session token
    sus timestamp tea = timez.now().to_string()
    sus random_part tea = cryptz.random_string(32)
    damn cryptz.sha256(timestamp + random_part)
}

slay generate_message_id() drip {
    damn timez.now()
}

slay cleanup_expired_sessions() yikes<drip> {
    ready (!db.initialized) {
        yikes "Database not initialized"
    }
    
    sus current_time drip = timez.now()
    sus sessions_dir tea = db.data_dir + "/sessions"
    sus files []tea = filez.list_files(sessions_dir) fam {
        yikes "Failed to list session files"
    }
    
    sus deleted_count drip = 0
    
    bestie (sus i drip = 0; i < files.length(); i++) {
        sus file_path tea = sessions_dir + "/" + files[i]
        sus session_json tea = filez.read_file(file_path) fam {
            next # Skip corrupted files
        }
        
        sus session Session = session_from_json(session_json) fam {
            next # Skip corrupted files
        }
        
        ready (session.expires_at < current_time) {
            filez.delete_file(file_path) fam {
                # Ignore delete errors
            }
            deleted_count = deleted_count + 1
        }
    }
    
    damn deleted_count
}
