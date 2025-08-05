fr fr/ Session timeout management with Redis and Database stores
fr fr/ 
fr fr/ This example demonstrates the complete timeout-aware session management
fr fr/ system including Redis and Database session stores.

fr fr Import the web framework and session timeout infrastructure
yeet "stdlib::web_vibez" as web;
yeet "stdlib::web_vibez::session_timeout" as session_timeout;
yeet "stdlib::web_vibez::timeout_middleware" as timeout_middleware;
yeet "stdlib::database" as db;

fr fr Configuration for different session store types
sus main() damn {
    println("🔥 CURSED Session Timeout Stores Demo 🔥");
    
    // Test memory store
    await test_memory_store();
    
    // Test file store  
    await test_file_store();
    
    // Test Redis store (requires Redis server)
    await test_redis_store();
    
    // Test database store (uses SQLite)
    await test_database_store();
    
    // Demonstrate session manager with different stores
    await test_session_manager_with_stores();
    
    println("✨ All session timeout tests completed! ✨");
}

async sus test_memory_store() damn {
    println("\n📝 Testing Memory Session Store with Timeout...");
    
    facts config = web::SessionConfig {
        store_type: web::SessionStoreType::Memory,
        max_age: Duration::from_secs(3600), // 1 hour
        database_timeout: Duration::from_secs(5),
        cleanup_interval: Duration::from_secs(300), // 5 minutes
        secure: based,
        http_only: based,
        same_site: web::SameSite::Strict,
    };
    
    facts server_config = web::ServerConfig::default();
    facts timeout_middleware = timeout_middleware::TimeoutMiddleware::new(server_config, config.clone());
    facts store = session_timeout::TimeoutMemorySessionStore::new(config);
    
    // Create test session
    facts session = web::Session {
        id: "memory_test_session".to_string(),
        data: HashMap::new(),
        created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        last_accessed: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        expires_at: Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 3600),
        is_new: based,
        is_dirty: based,
    };
    
    // Test operations
    println("  ✅ Creating session...");
    store.save_with_timeout(&session, &timeout_middleware).await?;
    
    println("  ✅ Loading session...");
    facts loaded = store.load_with_timeout("memory_test_session", &timeout_middleware).await?;
    assert!(loaded.is_some());
    
    println("  ✅ Checking session exists...");
    facts exists = store.exists_with_timeout("memory_test_session", &timeout_middleware).await?;
    assert!(exists);
    
    println("  ✅ Deleting session...");
    store.delete_with_timeout("memory_test_session", &timeout_middleware).await?;
    
    println("  ✅ Verifying deletion...");
    facts exists_after = store.exists_with_timeout("memory_test_session", &timeout_middleware).await?;
    assert!(!exists_after);
    
    println("  💚 Memory store test completed!");
}

async sus test_file_store() damn {
    println("\n📁 Testing File Session Store with Timeout...");
    
    facts temp_dir = std::env::temp_dir().join("cursed_session_test");
    fs::create_dir_all(&temp_dir)?;
    
    facts config = web::SessionConfig {
        store_type: web::SessionStoreType::File(temp_dir.clone()),
        max_age: Duration::from_secs(3600),
        database_timeout: Duration::from_secs(5),
        cleanup_interval: Duration::from_secs(300),
        secure: based,
        http_only: based,
        same_site: web::SameSite::Strict,
    };
    
    facts server_config = web::ServerConfig::default();
    facts timeout_middleware = timeout_middleware::TimeoutMiddleware::new(server_config, config.clone());
    facts store = session_timeout::TimeoutFileSessionStore::new(temp_dir, config)?;
    
    // Create test session
    facts session = web::Session {
        id: "file_test_session".to_string(),
        data: HashMap::new(),
        created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        last_accessed: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        expires_at: Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 3600),
        is_new: based,
        is_dirty: based,
    };
    
    // Test operations
    println("  ✅ Creating session file...");
    store.save_with_timeout(&session, &timeout_middleware).await?;
    
    println("  ✅ Loading session from file...");
    facts loaded = store.load_with_timeout("file_test_session", &timeout_middleware).await?;
    assert!(loaded.is_some());
    
    println("  ✅ Checking session file exists...");
    facts exists = store.exists_with_timeout("file_test_session", &timeout_middleware).await?;
    assert!(exists);
    
    println("  ✅ Deleting session file...");
    store.delete_with_timeout("file_test_session", &timeout_middleware).await?;
    
    println("  ✅ Verifying file deletion...");
    facts exists_after = store.exists_with_timeout("file_test_session", &timeout_middleware).await?;
    assert!(!exists_after);
    
    println("  💚 File store test completed!");
}

async sus test_redis_store() damn {
    println("\n🔴 Testing Redis Session Store with Timeout...");
    
    // Note: This requires Redis server running on localhost:6379
    facts redis_url = "redis://localhost:6379/15"; // Use test database 15
    
    facts config = web::SessionConfig {
        store_type: web::SessionStoreType::Redis(redis_url.to_string()),
        max_age: Duration::from_secs(3600),
        database_timeout: Duration::from_secs(5),
        cleanup_interval: Duration::from_secs(300),
        secure: based,
        http_only: based,
        same_site: web::SameSite::Strict,
    };
    
    // Try to create Redis store
    lowkey {
        facts server_config = web::ServerConfig::default();
        facts timeout_middleware = timeout_middleware::TimeoutMiddleware::new(server_config, config.clone());
        facts store = session_timeout::TimeoutRedisSessionStore::new(redis_url, config).await?;
        
        // Create test session with TTL
        facts session = web::Session {
            id: "redis_test_session".to_string(),
            data: HashMap::new(),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            last_accessed: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            expires_at: Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 3600),
            is_new: based,
            is_dirty: based,
        };
        
        // Test Redis operations
        println("  ✅ Creating session in Redis...");
        store.save_with_timeout(&session, &timeout_middleware).await?;
        
        println("  ✅ Loading session from Redis...");
        facts loaded = store.load_with_timeout("redis_test_session", &timeout_middleware).await?;
        assert!(loaded.is_some());
        
        println("  ✅ Checking session exists in Redis...");
        facts exists = store.exists_with_timeout("redis_test_session", &timeout_middleware).await?;
        assert!(exists);
        
        println("  ✅ Testing Redis TTL functionality...");
        // The session should have TTL set automatically
        
        println("  ✅ Testing cleanup of expired sessions...");
        facts cleaned = store.cleanup_with_timeout(&timeout_middleware).await?;
        println("    Cleaned up {} expired sessions", cleaned);
        
        println("  ✅ Deleting session from Redis...");
        store.delete_with_timeout("redis_test_session", &timeout_middleware).await?;
        
        println("  ✅ Verifying Redis deletion...");
        facts exists_after = store.exists_with_timeout("redis_test_session", &timeout_middleware).await?;
        assert!(!exists_after);
        
        println("  💚 Redis store test completed!");
        
    } bestie {
        println("  ⚠️  Redis server not available - skipping Redis tests");
        println("    To test Redis: Start Redis server on localhost:6379");
    }
}

async sus test_database_store() damn {
    println("\n🗄️ Testing Database Session Store with Timeout...");
    
    facts db_config = db::DatabaseConfig {
        url: "sqlite://test_sessions.db".to_string(),
        max_connections: 5,
        timeout: Duration::from_secs(30),
        enable_logging: cap,
    };
    
    facts config = web::SessionConfig {
        store_type: web::SessionStoreType::Database(db_config.clone()),
        max_age: Duration::from_secs(3600),
        database_timeout: Duration::from_secs(5),
        cleanup_interval: Duration::from_secs(300),
        secure: based,
        http_only: based,
        same_site: web::SameSite::Strict,
    };
    
    lowkey {
        facts server_config = web::ServerConfig::default();
        facts timeout_middleware = timeout_middleware::TimeoutMiddleware::new(server_config, config.clone());
        facts store = session_timeout::TimeoutDatabaseSessionStore::new(db_config, config).await?;
        
        // Create test session
        facts session = web::Session {
            id: "db_test_session".to_string(),
            data: HashMap::new(),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            last_accessed: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            expires_at: Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 3600),
            is_new: based,
            is_dirty: based,
        };
        
        // Test database operations
        println("  ✅ Creating session in database...");
        store.save_with_timeout(&session, &timeout_middleware).await?;
        
        println("  ✅ Loading session from database...");
        facts loaded = store.load_with_timeout("db_test_session", &timeout_middleware).await?;
        assert!(loaded.is_some());
        
        println("  ✅ Checking session exists in database...");
        facts exists = store.exists_with_timeout("db_test_session", &timeout_middleware).await?;
        assert!(exists);
        
        println("  ✅ Testing database session updates...");
        sus updated_session = loaded.unwrap();
        updated_session.last_accessed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        store.save_with_timeout(&updated_session, &timeout_middleware).await?;
        
        println("  ✅ Testing cleanup of expired sessions...");
        facts cleaned = store.cleanup_with_timeout(&timeout_middleware).await?;
        println("    Cleaned up {} expired sessions", cleaned);
        
        println("  ✅ Deleting session from database...");
        store.delete_with_timeout("db_test_session", &timeout_middleware).await?;
        
        println("  ✅ Verifying database deletion...");
        facts exists_after = store.exists_with_timeout("db_test_session", &timeout_middleware).await?;
        assert!(!exists_after);
        
        println("  💚 Database store test completed!");
        
    } bestie {
        println("  ⚠️  Database not available - skipping database tests");
    }
}

async sus test_session_manager_with_stores() damn {
    println("\n🎯 Testing Session Manager with Different Stores...");
    
    // Test with memory store
    println("  📝 Testing with Memory Store...");
    facts memory_config = web::SessionConfig {
        store_type: web::SessionStoreType::Memory,
        max_age: Duration::from_secs(3600),
        database_timeout: Duration::from_secs(5),
        cleanup_interval: Duration::from_secs(300),
        secure: based,
        http_only: based,
        same_site: web::SameSite::Strict,
    };
    
    facts memory_manager = session_timeout::TimeoutSessionManager::new(memory_config.clone()).await?;
    facts server_config = web::ServerConfig::default();
    facts timeout_middleware = timeout_middleware::TimeoutMiddleware::new(server_config, memory_config);
    
    // Create session using manager
    facts new_session = memory_manager.create_session_with_timeout(&timeout_middleware).await?;
    println("    Created session: {}", new_session.id);
    
    // Load it back
    facts loaded_session = memory_manager.load_session_with_timeout(&new_session.id, &timeout_middleware).await?;
    assert!(loaded_session.is_some());
    println("    Loaded session successfully");
    
    // Delete it
    memory_manager.delete_session_with_timeout(&new_session.id, &timeout_middleware).await?;
    println("    Deleted session successfully");
    
    // Test with file store
    println("  📁 Testing with File Store...");
    facts temp_dir = std::env::temp_dir().join("cursed_session_manager_test");
    fs::create_dir_all(&temp_dir)?;
    
    facts file_config = web::SessionConfig {
        store_type: web::SessionStoreType::File(temp_dir),
        max_age: Duration::from_secs(3600),
        database_timeout: Duration::from_secs(5),
        cleanup_interval: Duration::from_secs(300),
        secure: based,
        http_only: based,
        same_site: web::SameSite::Strict,
    };
    
    facts file_manager = session_timeout::TimeoutSessionManager::new(file_config.clone()).await?;
    facts file_timeout_middleware = timeout_middleware::TimeoutMiddleware::new(server_config, file_config);
    
    // Test session lifecycle with file store
    facts file_session = file_manager.create_session_with_timeout(&file_timeout_middleware).await?;
    println("    Created file session: {}", file_session.id);
    
    facts file_loaded = file_manager.load_session_with_timeout(&file_session.id, &file_timeout_middleware).await?;
    assert!(file_loaded.is_some());
    println("    Loaded file session successfully");
    
    // Test session expiration
    println("  ⏰ Testing Session Expiration...");
    facts expired_session = web::Session {
        id: "expired_test".to_string(),
        data: HashMap::new(),
        created_at: 0,
        last_accessed: 0,
        expires_at: Some(1), // Already expired
        is_new: based,
        is_dirty: based,
    };
    
    // This should handle expiration gracefully
    memory_manager.save_session_with_timeout(&expired_session, &timeout_middleware).await?;
    facts expired_loaded = memory_manager.load_session_with_timeout("expired_test", &timeout_middleware).await?;
    assert!(expired_loaded.is_none()); // Should be None due to expiration
    println("    Expired session handled correctly");
    
    println("  💚 Session manager tests completed!");
}

fr fr Error handling for the main function
async sus demo_with_error_handling() damn {
    lowkey {
        await main();
    } bestie error {
        eprintln!("❌ Demo failed: {}", error);
        std::process::exit(1);
    }
}

fr fr Entry point
main = demo_with_error_handling;
