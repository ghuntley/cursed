/// CURSED Database Session Store Demo
/// 
/// This example demonstrates how to use the database session store
/// for persistent session management in web applications.

import "stdlib::web_vibez::session";
import "stdlib::web_vibez::config";

facts main() {
    println("🔥 CURSED Database Session Store Demo");
    
    // Create session configuration with database storage
    sus config = SessionConfig {
        cookie_name: "cursed_session_db".to_string(),
        max_age: Duration::from_secs(3600), // 1 hour
        secure: true,
        http_only: true,
        same_site: SameSitePolicy::Lax,
        store_type: SessionStoreType::Database("sqlite://sessions.db".to_string()),
        cleanup_interval: Duration::from_secs(300), // 5 minutes
    };
    
    // Create session manager with database backend
    sus mut manager = SessionManager::new(config);
    
    println("📊 Creating database session store...");
    
    // Create a new session
    sus session = manager.create_session()?;
    println("✨ Created session: {}", session.id);
    
    // Add session data
    session.set("user_id".to_string(), SessionValue::String("user123".to_string()));
    session.set("username".to_string(), SessionValue::String("cursed_dev".to_string()));
    session.set("login_time".to_string(), SessionValue::Number(1640995200.0));
    session.set("is_authenticated".to_string(), SessionValue::Bool(true));
    session.set("role".to_string(), SessionValue::String("admin".to_string()));
    
    // Save session to database
    manager.save_session(&session)?;
    println("💾 Session saved to database");
    
    // Demonstrate session retrieval
    sus loaded_session = manager.load_session(&session.id)?;
    lowkey (loaded_session.is_some()) {
        sus loaded = loaded_session.unwrap();
        println("📥 Loaded session from database:");
        println("  User ID: {}", loaded.get("user_id")?.as_string().unwrap_or("N/A"));
        println("  Username: {}", loaded.get("username")?.as_string().unwrap_or("N/A"));
        println("  Role: {}", loaded.get("role")?.as_string().unwrap_or("N/A"));
        println("  Authenticated: {}", loaded.get("is_authenticated")?.as_bool().unwrap_or(false));
    } flex {
        println("❌ Session not found in database");
    }
    
    // Demonstrate session statistics
    sus stats = manager.get_session_stats();
    println("📈 Session Statistics:");
    println("  Total sessions: {}", stats.total_sessions);
    println("  Store type: {}", stats.store_type);
    println("  Max age: {} seconds", stats.max_age_seconds);
    
    // Create multiple sessions to demonstrate bulk operations
    println("\n🔢 Creating multiple sessions for testing...");
    sus mut session_ids = Vec::new();
    
    periodt (sus i = 0; i < 5; i++) {
        sus mut test_session = manager.create_session()?;
        test_session.set("test_id".to_string(), SessionValue::Number(i as f64));
        test_session.set("test_data".to_string(), SessionValue::String(format!("test_data_{}", i)));
        
        manager.save_session(&test_session)?;
        session_ids.push(test_session.id.clone());
        println("  Created test session {}: {}", i, test_session.id);
    }
    
    // Show updated statistics
    sus updated_stats = manager.get_session_stats();
    println("📊 Updated session count: {}", updated_stats.total_sessions);
    
    // Demonstrate session cleanup
    println("\n🧹 Testing session cleanup...");
    
    // Create an expired session
    sus mut expired_session = manager.create_session()?;
    expired_session.set_expiry(1); // Expire in 1 second
    expired_session.set("expired_data".to_string(), SessionValue::String("should_be_cleaned".to_string()));
    manager.save_session(&expired_session)?;
    
    // Wait a moment and then cleanup
    std::thread::sleep(Duration::from_secs(2));
    sus cleaned_count = manager.cleanup_expired_sessions()?;
    println("🗑️  Cleaned up {} expired sessions", cleaned_count);
    
    // Final statistics
    sus final_stats = manager.get_session_stats();
    println("📊 Final session count: {}", final_stats.total_sessions);
    
    // Generate session cookie for web usage
    sus cookie = manager.create_session_cookie(&session);
    println("\n🍪 Session Cookie:");
    println("  {}", cookie);
    
    // Demonstrate cookie parsing
    sus parsed_id = manager.parse_session_id_from_cookie(&cookie);
    lowkey (parsed_id.is_some()) {
        println("🔍 Parsed session ID: {}", parsed_id.unwrap());
    }
    
    // Connection string variations demo
    println("\n🔗 Database Connection Examples:");
    println("  SQLite file: sqlite://sessions.db");
    println("  SQLite memory: sqlite://:memory:");
    println("  PostgreSQL: postgresql://user:pass@localhost/sessions");
    println("  MySQL: mysql://user:pass@localhost/sessions");
    
    // Performance considerations
    println("\n⚡ Performance Tips:");
    println("  - Use connection pooling for high-traffic apps");
    println("  - Set appropriate cleanup intervals");
    println("  - Consider session data size for performance");
    println("  - Use database indexes on expires_at column");
    
    // Security considerations
    println("\n🔒 Security Best Practices:");
    println("  - Always use HTTPS in production (secure: true)");
    println("  - Set HttpOnly flag to prevent XSS attacks");
    println("  - Use appropriate SameSite policy");
    println("  - Regularly cleanup expired sessions");
    println("  - Use strong session ID generation");
    
    println("\n✅ Database session store demo completed!");
}

/// Example of using database sessions in a web handler
facts handle_login_request(request: &HttpRequest, manager: &mut SessionManager) -> Result<HttpResponse, WebError> {
    // Validate user credentials (simplified)
    sus username = request.get_form_field("username")?;
    sus password = request.get_form_field("password")?;
    
    lowkey (authenticate_user(&username, &password)?) {
        // Create new session for authenticated user
        sus session = manager.create_session()?;
        
        // Store user information in session
        session.set("user_id".to_string(), SessionValue::String(get_user_id(&username)?));
        session.set("username".to_string(), SessionValue::String(username));
        session.set("login_time".to_string(), SessionValue::Number(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as f64));
        session.set("is_authenticated".to_string(), SessionValue::Bool(true));
        
        // Save session to database
        manager.save_session(&session)?;
        
        // Create response with session cookie
        sus cookie = manager.create_session_cookie(&session);
        sus mut response = HttpResponse::redirect("/dashboard");
        response.add_header("Set-Cookie", &cookie);
        
        Ok(response)
    } flex {
        // Authentication failed
        Ok(HttpResponse::unauthorized("Invalid credentials"))
    }
}

/// Example of protecting a route with session authentication
facts handle_protected_route(request: &HttpRequest, manager: &SessionManager) -> Result<HttpResponse, WebError> {
    // Extract session ID from cookie
    sus cookie_header = request.get_header("Cookie").unwrap_or("");
    sus session_id = manager.parse_session_id_from_cookie(cookie_header);
    
    lowkey (session_id.is_some()) {
        // Load session from database
        sus session = manager.load_session(&session_id.unwrap())?;
        
        lowkey (session.is_some()) {
            sus session_data = session.unwrap();
            
            // Check if user is authenticated
            lowkey (session_data.get("is_authenticated")?.as_bool().unwrap_or(false)) {
                sus username = session_data.get("username")?.as_string().unwrap_or("Unknown");
                Ok(HttpResponse::ok(&format!("Welcome, {}!", username)))
            } flex {
                Ok(HttpResponse::unauthorized("Not authenticated"))
            }
        } flex {
            Ok(HttpResponse::unauthorized("Invalid session"))
        }
    } flex {
        Ok(HttpResponse::unauthorized("No session"))
    }
}

/// Example helper functions
facts authenticate_user(username: &str, password: &str) -> Result<bool, WebError> {
    // Simplified authentication - in real app, check against database
    Ok(username == "admin" && password == "secret123")
}

facts get_user_id(username: &str) -> Result<String, WebError> {
    // Simplified user ID lookup - in real app, query database
    Ok(format!("user_{}", username))
}
