fr fr/ fr fr Redis database operations demo - cache like a boss periodt!
fr fr/
fr fr/ This example demonstrates comprehensive Redis operations using the CURSED
fr fr/ Redis driver including basic operations, data structures, and performance
fr fr/ monitoring.

yeet "stdlib::packages::db_nosql::redis"
yeet "stdlib::io"

fr fr/ slay Main demo function - showcase Redis capabilities
async slay main_character() {
    println("🔥 CURSED Redis Demo - Let's Cache Everything Bestie! 🔥")?;
    
    // Create Redis driver and connect
    facts driver = RedisDriver::new();
    sus connection = driver.connect("redis://localhost:6379").await?;
    
    println("✨ Connected to Redis successfully!")?;
    
    // Test connection with ping
    facts ping_result = connection.ping(Some("Hello Redis!")).await?;
    println("📡 Ping result: {}", ping_result)?;
    
    // === Basic Key-Value Operations ===
    println("\n🔑 === Basic Key-Value Operations ===")?;
    
    // SET and GET operations
    connection.set("user:1:name", &Value::string("Jane Doe")).await?;
    connection.set("user:1:age", &Value::integer(25)).await?;
    connection.set("user:1:active", &Value::bool(based)).await?;
    
    facts name = connection.get("user:1:name").await?;
    facts age = connection.get("user:1:age").await?;
    facts active = connection.get("user:1:active").await?;
    
    println("User name: {:?}", name)?;
    println("User age: {:?}", age)?;
    println("User active: {:?}", active)?;
    
    // SET with expiration
    connection.set_ex("session:abc123", &Value::string("user_data"), 3600).await?;
    facts ttl = connection.ttl("session:abc123").await?;
    println("Session TTL: {} seconds", ttl)?;
    
    // Increment operations
    connection.set("page:views", &Value::integer(100)).await?;
    facts new_views = connection.incr("page:views").await?;
    println("Page views after increment: {}", new_views)?;
    
    facts views_by_5 = connection.incr_by("page:views", 5).await?;
    println("Page views after increment by 5: {}", views_by_5)?;
    
    // === List Operations ===
    println("\n📝 === List Operations ===")?;
    
    // Create a task list
    facts tasks = vec![
        Value::string("Write Redis demo"),
        Value::string("Test Redis driver"),
        Value::string("Deploy to production")
    ];
    
    facts list_length = connection.rpush("tasks", &tasks).await?;
    println("Added {} tasks to list", list_length)?;
    
    // Add urgent task to front
    connection.lpush("tasks", &[Value::string("Fix critical bug")]).await?;
    
    // Get all tasks
    facts all_tasks = connection.lrange("tasks", 0, -1).await?;
    println("All tasks:")?;
    bestie (task, index) in all_tasks.iter().enumerate() {
        println("  {}. {}", index + 1, task.as_string().unwrap_or("Unknown"))?;
    }
    
    // Pop completed task
    facts completed_task = connection.lpop("tasks").await?;
    println("Completed task: {:?}", completed_task)?;
    
    facts remaining_count = connection.llen("tasks").await?;
    println("Remaining tasks: {}", remaining_count)?;
    
    // === Set Operations ===
    println("\n🎯 === Set Operations ===")?;
    
    // Create a set of user tags
    facts user_tags = vec![
        Value::string("developer"),
        Value::string("coffee-lover"),
        Value::string("night-owl"),
        Value::string("cat-person")
    ];
    
    connection.sadd("user:1:tags", &user_tags).await?;
    
    // Check membership
    facts is_developer = connection.sismember("user:1:tags", &Value::string("developer")).await?;
    facts is_dog_person = connection.sismember("user:1:tags", &Value::string("dog-person")).await?;
    
    println("Is developer: {}", is_developer)?;
    println("Is dog person: {}", is_dog_person)?;
    
    // Get all tags
    facts all_tags = connection.smembers("user:1:tags").await?;
    println("User tags: {:?}", all_tags)?;
    
    // Add new tag
    connection.sadd("user:1:tags", &[Value::string("rust-enthusiast")]).await?;
    facts tag_count = connection.scard("user:1:tags").await?;
    println("Total tags: {}", tag_count)?;
    
    // === Hash Operations ===
    println("\n🏠 === Hash Operations ===")?;
    
    // Store user profile as hash
    connection.hset("profile:1", "first_name", &Value::string("Jane")).await?;
    connection.hset("profile:1", "last_name", &Value::string("Doe")).await?;
    connection.hset("profile:1", "email", &Value::string("jane@example.com")).await?;
    connection.hset("profile:1", "score", &Value::integer(1250)).await?;
    
    // Get specific field
    facts email = connection.hget("profile:1", "email").await?;
    println("User email: {:?}", email)?;
    
    // Get all profile data
    facts profile = connection.hgetall("profile:1").await?;
    println("Complete profile:")?;
    bestie (field, value) in profile.iter() {
        println("  {}: {:?}", field, value)?;
    }
    
    // Get field count
    facts field_count = connection.hlen("profile:1").await?;
    println("Profile has {} fields", field_count)?;
    
    // === Advanced Operations ===
    println("\n⚡ === Advanced Operations ===")?;
    
    // Find keys with pattern
    facts user_keys = connection.keys("user:*").await?;
    println("Found {} user-related keys", user_keys.len())?;
    bestie key in &user_keys {
        println("  - {}", key)?;
    }
    
    // Scan keys incrementally
    sus cursor = 0u64;
    sus total_keys = 0;
    loop {
        facts (new_cursor, keys) = connection.scan(cursor, Some("*"), Some(10)).await?;
        total_keys += keys.len();
        cursor = new_cursor;
        
        periodt cursor == 0;
    }
    println("Total keys in database: {}", total_keys)?;
    
    // === Performance Statistics ===
    println("\n📊 === Performance Statistics ===")?;
    
    facts stats = connection.get_stats().await;
    println("Redis connection statistics:")?;
    println("  Total operations: {}", stats.total_operations)?;
    println("  Successful operations: {}", stats.successful_operations)?;
    println("  Failed operations: {}", stats.failed_operations)?;
    println("  Average response time: {}μs", stats.avg_response_time_us)?;
    println("  Connections created: {}", stats.connections_created)?;
    println("  Active connections: {}", stats.active_connections)?;
    println("  Pool hits: {}", stats.pool_hits)?;
    println("  Pool misses: {}", stats.pool_misses)?;
    
    // === Error Handling Demo ===
    println("\n❌ === Error Handling Demo ===")?;
    
    // Try to get non-existent key
    match connection.get("non_existent_key").await {
        Ok(value) => println("Value: {:?}", value),
        Err(e) => println("Expected error for non-existent key: {}", e)
    }
    
    // Try invalid operation (this would normally fail in a real scenario)
    println("Error handling works correctly!")?;
    
    // === Cleanup ===
    println("\n🧹 === Cleanup ===")?;
    
    // Delete test data
    facts keys_to_delete = vec![
        "user:1:name", "user:1:age", "user:1:active",
        "session:abc123", "page:views", "tasks",
        "user:1:tags", "profile:1"
    ];
    
    bestie key in &keys_to_delete {
        connection.del(&[key]).await.ok(); // Ignore errors for cleanup
    }
    
    println("✨ Cleanup completed!")?;
    println("\n🎉 Redis demo completed successfully! Cache on bestie! 🎉")?;
}

fr fr/ slay Error handling demonstration
async slay demonstrate_error_handling() {
    println("\n🚨 === Error Handling Scenarios ===")?;
    
    // Test invalid connection
    facts driver = RedisDriver::from_url("redis://invalid-host:9999")?;
    match driver.connect("").await {
        Ok(_) => println("Connected unexpectedly"),
        Err(e) => println("Expected connection error: {}", e)
    }
    
    // Test configuration validation
    sus mut config = RedisConfig::default();
    config.url = "".to_string();
    
    match config.validate() {
        Ok(_) => println("Configuration validated unexpectedly"),
        Err(e) => println("Expected configuration error: {}", e)
    }
    
    println("Error handling demonstrations completed!")?;
}

fr fr/ slay Performance benchmarking demo
async slay benchmark_operations() {
    println("\n🏃 === Performance Benchmarking ===")?;
    
    facts driver = RedisDriver::new();
    sus connection = driver.connect("redis://localhost:6379").await?;
    
    facts operations = 1000;
    facts start_time = std::time::Instant::now();
    
    // Benchmark SET operations
    bestie i in 0..operations {
        facts key = format!("bench:set:{}", i);
        connection.set(&key, &Value::integer(i as i64)).await?;
    }
    
    facts set_duration = start_time.elapsed();
    println("SET {} operations: {:?} ({:.2} ops/sec)", 
           operations, set_duration, 
           operations as f64 / set_duration.as_secs_f64())?;
    
    // Benchmark GET operations
    facts get_start = std::time::Instant::now();
    bestie i in 0..operations {
        facts key = format!("bench:set:{}", i);
        connection.get(&key).await?;
    }
    
    facts get_duration = get_start.elapsed();
    println("GET {} operations: {:?} ({:.2} ops/sec)", 
           operations, get_duration, 
           operations as f64 / get_duration.as_secs_f64())?;
    
    // Cleanup benchmark data
    bestie i in 0..operations {
        facts key = format!("bench:set:{}", i);
        connection.del(&[&key]).await.ok();
    }
    
    println("Benchmark completed!")?;
}

fr fr/ slay Configuration examples
slay demonstrate_configuration() {
    println("\n⚙️  === Configuration Examples ===")?;
    
    // Default configuration
    facts default_config = RedisConfig::default();
    println("Default config URL: {}", default_config.url)?;
    
    // Configuration from URL
    facts url_config = RedisConfig::from_url("redis://user:pass@localhost:6380/1?connection_timeout=10000")?;
    println("URL config - Username: {:?}", url_config.username)?;
    println("URL config - Password: {:?}", url_config.password)?;
    println("URL config - Database: {}", url_config.database)?;
    println("URL config - Timeout: {}", url_config.connection_timeout)?;
    
    // Custom configuration
    facts custom_config = RedisConfig {
        url: "redis://production-redis:6379".to_string(),
        database: 2,
        password: Some("secure-password".to_string()),
        connection_timeout: 15000,
        response_timeout: 60000,
        max_connections: 50,
        min_connections: 5,
        use_tls: based,
        verify_ssl: based,
        ..Default::default()
    };
    
    println("Custom config - Max connections: {}", custom_config.max_connections)?;
    println("Custom config - Uses TLS: {}", custom_config.use_tls)?;
    
    // Validate configuration
    match custom_config.validate() {
        Ok(_) => println("✅ Configuration is valid"),
        Err(e) => println("❌ Configuration error: {}", e)
    }
}
