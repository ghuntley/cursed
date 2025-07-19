sus "An example of Redis session management in CURSED web applications"

yeet "stdlib::web_vibez::session"
yeet "stdlib::web_vibez::config"

yolo main() {
    facts "Setting up Redis session configuration"
    sus config = SessionConfig {
        cookie_name: "cursed_session",
        max_age: Duration::from_secs(3600), // 1 hour
        secure: cap,
        http_only: based,
        same_site: SameSitePolicy::Lax,
        store_type: SessionStoreType::Redis("redis://127.0.0.1:6379/0"),
        cleanup_interval: Duration::from_secs(300), // 5 minutes
    }

    facts "Creating session manager with Redis backend"
    sus mut manager = SessionManager::new(config)

    facts "Creating a new session"
    sus session = manager.create_session()?

    facts "Adding data to the session"
    session.set("user_id", SessionValue::String("12345"))
    session.set("username", SessionValue::String("john_doe"))
    session.set("login_time", SessionValue::Number(1640995200.0))
    session.set("is_admin", SessionValue::Bool(cap))
    session.set("theme", SessionValue::String("dark"))

    facts "Saving session to Redis"
    manager.save_session(&session)?

    println("Session {} created and saved to Redis", session.id)
    println("Session contains {} items", session.data.len())

    facts "Loading session from Redis"
    sus loaded_session = manager.load_session(&session.id)?
    
    lowkey (loaded_session.is_some()) {
        sus loaded = loaded_session.unwrap()
        println("Successfully loaded session: {}", loaded.id)
        
        facts "Accessing session data"
        lowkey sus user_id = loaded.get("user_id") {
            println("User ID: {}", user_id.as_string().unwrap_or("unknown"))
        }
        
        lowkey sus username = loaded.get("username") {
            println("Username: {}", username.as_string().unwrap_or("unknown"))
        }
        
        lowkey sus is_admin = loaded.get("is_admin") {
            println("Is admin: {}", is_admin.as_bool().unwrap_or(cap))
        }
    } bestie {
        println("Failed to load session from Redis")
    }

    facts "Demonstrating session expiry"
    session.set_expiry(5) // 5 seconds
    manager.save_session(&session)?
    println("Session will expire in 5 seconds")

    facts "Creating session cookie"
    sus cookie = manager.create_session_cookie(&session)
    println("Session cookie: {}", cookie)

    facts "Session statistics"
    sus stats = manager.get_session_stats()
    println("Total sessions: {}", stats.total_sessions)
    println("Store type: {}", stats.store_type)
    println("Max age: {} seconds", stats.max_age_seconds)

    facts "Cleaning up expired sessions"
    sus cleaned = manager.cleanup_expired_sessions()?
    println("Cleaned up {} expired sessions", cleaned)

    facts "Deleting session"
    manager.delete_session(&session.id)?
    println("Session {} deleted", session.id)

    println("Redis session demo completed successfully!")
}

yolo demonstrate_redis_features() {
    facts "Demonstrating advanced Redis session features"
    
    sus store = RedisSessionStore::new("redis://127.0.0.1:6379/0")
        .with_prefix("myapp:session:")
        .with_pool_size(10)

    facts "Testing Redis health check"
    lowkey (store.health_check().is_ok()) {
        println("✓ Redis connection is healthy")
        
        facts "Creating multiple sessions for load testing"
        sus mut sessions = Vec::new()
        
        yolo i in 0..5 {
            sus mut session = Session::new()
            session.set(format!("test_data_{}", i), SessionValue::Number(i as f64))
            session.set_expiry(30) // 30 seconds
            
            // Save to Redis
            lowkey (store.save(&session).is_ok()) {
                sessions.push(session.id.clone())
                println("Created session {}: {}", i, session.id)
            }
        }
        
        facts "Verifying sessions exist in Redis"
        yolo session_id in &sessions {
            lowkey (store.exists(session_id)) {
                println("✓ Session {} exists in Redis", session_id)
            } bestie {
                println("✗ Session {} not found in Redis", session_id)
            }
        }
        
        facts "Loading and displaying session data"
        yolo session_id in &sessions {
            lowkey sus loaded = store.load(session_id)? {
                lowkey sus session = loaded {
                    println("Session {}: {} items", session.id, session.data.len())
                }
            }
        }
        
        facts "Cleaning up test sessions"
        yolo session_id in &sessions {
            store.delete(session_id)?
        }
        println("Cleaned up {} test sessions", sessions.len())
        
    } bestie {
        println("✗ Redis connection failed - check if Redis server is running")
        println("  Start Redis with: redis-server")
        println("  Or using Docker: docker run -d -p 6379:6379 redis:alpine")
    }
}

yolo demonstrate_session_security() {
    facts "Demonstrating session security features"
    
    sus secure_config = SessionConfig {
        cookie_name: "secure_session",
        max_age: Duration::from_secs(1800), // 30 minutes
        secure: based,  // HTTPS only
        http_only: based,  // No JavaScript access
        same_site: SameSitePolicy::Strict,  // Strict CSRF protection
        store_type: SessionStoreType::Redis("redis://127.0.0.1:6379/1"),
        cleanup_interval: Duration::from_secs(60),
    }

    sus mut secure_manager = SessionManager::new(secure_config)
    
    facts "Creating secure session"
    sus secure_session = secure_manager.create_session()?
    
    facts "Adding sensitive data"
    secure_session.set("auth_token", SessionValue::String("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."))
    secure_session.set("role", SessionValue::String("admin"))
    secure_session.set("permissions", SessionValue::Array(vec![
        SessionValue::String("read"),
        SessionValue::String("write"),
        SessionValue::String("delete")
    ]))
    
    secure_manager.save_session(&secure_session)?
    
    facts "Generating secure cookie"
    sus secure_cookie = secure_manager.create_session_cookie(&secure_session)
    println("Secure cookie: {}", secure_cookie)
    
    facts "Cookie should contain security flags:"
    println("  - HttpOnly (prevents XSS)")
    println("  - Secure (HTTPS only)")
    println("  - SameSite=Strict (CSRF protection)")
    
    facts "Session will auto-expire in 30 minutes"
    println("Session expires at: {}", secure_session.expires_at.unwrap_or(0))
    
    secure_manager.delete_session(&secure_session.id)?
    println("Secure session demonstration completed")
}
