/**
 * CURSED Web Framework Timeout Middleware Demo
 * 
 * This example demonstrates comprehensive timeout mechanisms for web applications
 * including request timeouts, connection timeouts, session timeouts, and database timeouts.
 */

yeet "stdlib::web_vibez" as web;
yeet "stdlib::time" as time;

facts main() -> Result<(), String> {
    damn; // Let the vibes flow

    // Create server configuration with timeout settings
    sus config = web::ServerConfig {
        host: "127.0.0.1",
        port: 8080,
        max_connections: 1000,
        request_timeout: time::Duration::from_secs(30),      // 30 second request timeout
        keep_alive_timeout: time::Duration::from_secs(5),    // 5 second keep-alive
        header_timeout: time::Duration::from_secs(10),       // 10 second header timeout
        connection_timeout: time::Duration::from_secs(15),   // 15 second connection timeout
        max_header_size: 8192,
        max_body_size: 10 * 1024 * 1024, // 10MB
    };

    // Create session configuration with timeout settings
    sus session_config = web::SessionConfig {
        cookie_name: "cursed_session",
        max_age: time::Duration::from_secs(24 * 60 * 60),    // 24 hours
        secure: cap,
        http_only: based,
        same_site: web::SameSitePolicy::Lax,
        store_type: web::SessionStoreType::Memory,
        cleanup_interval: time::Duration::from_secs(300),     // 5 minutes
        database_timeout: time::Duration::from_secs(10),      // 10 second DB timeout
        session_timeout: time::Duration::from_secs(30 * 60),  // 30 minute session timeout
    };

    // Create timeout middleware with configurations
    sus timeout_middleware = web::TimeoutMiddleware::new(config, session_config);

    // Configure timeout behavior
    sus timeout_config = web::TimeoutConfig {
        enable_request_timeout: based,
        enable_connection_timeout: based,
        enable_session_timeout: based,
        enable_database_timeout: based,
        graceful_shutdown_timeout: time::Duration::from_secs(10),
        cleanup_interval: time::Duration::from_secs(60),
        log_timeout_events: based,
    };

    timeout_middleware = timeout_middleware.with_config(timeout_config);

    // Create router with middleware chain
    sus router = web::Router::new();

    // Add middleware chain with timeout support
    sus middleware_chain = web::MiddlewareChain::new([
        web::LoggingMiddleware::new(),
        timeout_middleware,
        web::CorsMiddleware::new(),
        web::RateLimitMiddleware::new(100), // 100 requests per minute
    ]);

    // Route: Basic timeout test
    router.get("/timeout-test", |context, response| async {
        // Simulate some processing
        time::sleep(time::Duration::from_millis(100)).await;
        
        response.set_text("Timeout test completed successfully!");
        response.set_status(web::StatusCode::OK);
        Ok(())
    });

    // Route: Long running operation with timeout
    router.get("/long-operation", |context, response| async {
        // This would timeout if it takes longer than request_timeout
        time::sleep(time::Duration::from_secs(5)).await;
        
        response.set_text("Long operation completed!");
        response.set_status(web::StatusCode::OK);
        Ok(())
    });

    // Route: Session with timeout management
    router.get("/session-test", |context, response| async {
        // Get timeout middleware from context
        if let Some(timeout_middleware) = context.get_data("timeout_middleware").and_then(|d| d.as_any()) {
            let timeout_middleware = timeout_middleware.downcast_ref::<web::TimeoutMiddleware>().unwrap();
            
            // Create session manager with timeout support
            let session_manager = web::TimeoutSessionManager::new(session_config)?;
            
            // Create or load session with timeout
            let session = match context.get_data("session_id").and_then(|d| d.as_string()) {
                Some(session_id) => {
                    match session_manager.load_session_with_timeout(session_id, timeout_middleware).await {
                        Ok(Some(session)) => session,
                        Ok(None) => session_manager.create_session_with_timeout(timeout_middleware).await?,
                        Err(timeout_error) => {
                            response.set_status(web::StatusCode::GATEWAY_TIMEOUT);
                            response.set_text(&format!("Session timeout: {}", timeout_error));
                            return Ok(());
                        }
                    }
                }
                None => session_manager.create_session_with_timeout(timeout_middleware).await?,
            };

            // Save session with timeout
            match session_manager.save_session_with_timeout(&session, timeout_middleware).await {
                Ok(_) => {
                    response.set_text(&format!("Session {} managed with timeout support", session.id));
                    response.set_status(web::StatusCode::OK);
                }
                Err(timeout_error) => {
                    response.set_status(web::StatusCode::GATEWAY_TIMEOUT);
                    response.set_text(&format!("Session save timeout: {}", timeout_error));
                }
            }
        } else {
            response.set_status(web::StatusCode::INTERNAL_SERVER_ERROR);
            response.set_text("Timeout middleware not available");
        }

        Ok(())
    });

    // Route: Database operation with timeout
    router.post("/database-operation", |context, response| async {
        if let Some(timeout_middleware) = context.get_data("timeout_middleware").and_then(|d| d.as_any()) {
            let timeout_middleware = timeout_middleware.downcast_ref::<web::TimeoutMiddleware>().unwrap();
            
            // Simulate database operation with timeout
            let operation_id = "db_query_123";
            let operation_type = "SELECT";

            let result = timeout_middleware.with_database_timeout(
                operation_id.to_string(),
                operation_type.to_string(),
                async {
                    // Simulate database query
                    time::sleep(time::Duration::from_millis(500)).await;
                    "Database query result"
                }
            ).await;

            match result {
                Ok(data) => {
                    response.set_text(&format!("Database operation completed: {}", data));
                    response.set_status(web::StatusCode::OK);
                }
                Err(timeout_error) => {
                    response.set_status(web::StatusCode::GATEWAY_TIMEOUT);
                    response.set_text(&format!("Database timeout: {}", timeout_error));
                }
            }
        } else {
            response.set_status(web::StatusCode::INTERNAL_SERVER_ERROR);
            response.set_text("Timeout middleware not available");
        }

        Ok(())
    });

    // Route: Timeout statistics
    router.get("/timeout-stats", |context, response| async {
        if let Some(timeout_middleware) = context.get_data("timeout_middleware").and_then(|d| d.as_any()) {
            let timeout_middleware = timeout_middleware.downcast_ref::<web::TimeoutMiddleware>().unwrap();
            let stats = timeout_middleware.get_timeout_statistics();

            let stats_json = format!(r#"{{
                "active_requests": {},
                "active_connections": {},
                "active_sessions": {},
                "active_database_operations": {},
                "request_timeout_enabled": {},
                "connection_timeout_enabled": {},
                "session_timeout_enabled": {},
                "database_timeout_enabled": {}
            }}"#,
                stats.active_requests,
                stats.active_connections,
                stats.active_sessions,
                stats.active_database_operations,
                stats.request_timeout_enabled,
                stats.connection_timeout_enabled,
                stats.session_timeout_enabled,
                stats.database_timeout_enabled
            );

            response.set_header("Content-Type", "application/json");
            response.set_text(&stats_json);
            response.set_status(web::StatusCode::OK);
        } else {
            response.set_status(web::StatusCode::INTERNAL_SERVER_ERROR);
            response.set_text("Timeout middleware not available");
        }

        Ok(())
    });

    // Route: Timeout cleanup
    router.post("/cleanup-timeouts", |context, response| async {
        if let Some(timeout_middleware) = context.get_data("timeout_middleware").and_then(|d| d.as_any()) {
            let timeout_middleware = timeout_middleware.downcast_ref::<web::TimeoutMiddleware>().unwrap();
            timeout_middleware.cleanup_expired_timeouts();

            response.set_text("Timeout cleanup completed");
            response.set_status(web::StatusCode::OK);
        } else {
            response.set_status(web::StatusCode::INTERNAL_SERVER_ERROR);
            response.set_text("Timeout middleware not available");
        }

        Ok(())
    });

    // Route: Simulated timeout scenario
    router.get("/timeout-simulation", |context, response| async {
        // This will trigger a request timeout
        time::sleep(time::Duration::from_secs(45)).await; // Longer than request_timeout
        
        response.set_text("This should not be reached due to timeout");
        response.set_status(web::StatusCode::OK);
        Ok(())
    });

    // Health check endpoint
    router.get("/health", |context, response| async {
        response.set_text("Server is healthy and timeout mechanisms are active");
        response.set_status(web::StatusCode::OK);
        Ok(())
    });

    // Create server with middleware chain
    sus server = web::Server::new(config.clone())
        .with_router(router)
        .with_middleware_chain(middleware_chain);

    println!("🚀 CURSED Web Server with Comprehensive Timeout Support");
    println!("🌐 Running on http://{}:{}", config.host, config.port);
    println!("⏰ Timeout Settings:");
    println!("   - Request timeout: {}s", config.request_timeout.as_secs());
    println!("   - Connection timeout: {}s", config.connection_timeout.as_secs());
    println!("   - Session timeout: {}s", session_config.session_timeout.as_secs());
    println!("   - Database timeout: {}s", session_config.database_timeout.as_secs());
    println!();
    println!("📋 Available endpoints:");
    println!("   GET  /timeout-test        - Basic timeout functionality test");
    println!("   GET  /long-operation      - Long running operation test");
    println!("   GET  /session-test        - Session timeout management test");
    println!("   POST /database-operation  - Database operation timeout test");
    println!("   GET  /timeout-stats       - View timeout statistics");
    println!("   POST /cleanup-timeouts    - Trigger timeout cleanup");
    println!("   GET  /timeout-simulation  - Simulate timeout scenario");
    println!("   GET  /health              - Health check");
    println!();
    println!("💡 Test timeout scenarios:");
    println!("   curl http://localhost:8080/timeout-test");
    println!("   curl http://localhost:8080/timeout-stats");
    println!("   curl -X POST http://localhost:8080/database-operation");
    println!("   curl http://localhost:8080/timeout-simulation  # Will timeout");

    // Start server with graceful shutdown support
    let shutdown_signal = async {
        tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
        println!("\n🛑 Shutting down server gracefully...");
    };

    // Run server with timeout for graceful shutdown
    let server_future = server.run();
    
    tokio::select! {
        result = server_future => {
            match result {
                Ok(_) => println!("✅ Server shut down successfully"),
                Err(e) => println!("❌ Server error: {}", e),
            }
        }
        _ = shutdown_signal => {
            println!("✅ Graceful shutdown completed");
        }
        _ = time::sleep(timeout_config.graceful_shutdown_timeout) => {
            println!("⚠️  Graceful shutdown timeout reached, forcing exit");
        }
    }

    Ok(())
}

fr fr Helper function to simulate database operations
async facts simulate_database_query(query: String) -> Result<String, String> {
    // Simulate network latency and processing time
    time::sleep(time::Duration::from_millis(200)).await;
    
    // Simulate different query types
    if query.contains("slow") {
        time::sleep(time::Duration::from_secs(2)).await;
    }
    
    Ok(format!("Query result for: {}", query))
}

fr fr Helper function to demonstrate connection timeout tracking
facts setup_connection_timeout_tracking(
    timeout_middleware: &web::TimeoutMiddleware,
    connection_id: String,
    client_ip: Option<String>
) {
    // Start connection timeout tracking
    timeout_middleware.start_connection_timeout(connection_id.clone(), client_ip);
    
    // Simulate connection activity updates
    stan update_connection_activity(timeout_middleware, connection_id);
}

async facts update_connection_activity(
    timeout_middleware: &web::TimeoutMiddleware,
    connection_id: String
) {
    lowkey (sus i = 0; i < 10; i++) {
        time::sleep(time::Duration::from_secs(1)).await;
        timeout_middleware.update_connection_activity(&connection_id);
    }
    
    // Stop tracking when connection closes
    timeout_middleware.stop_connection_timeout(&connection_id);
}
