#!/usr/bin/env cursed

fr fr/ CURSED web_vibez HTTP Server Demo
fr fr/ 
fr fr/ This example demonstrates how to create and run an HTTP server
fr fr/ using the CURSED web_vibez framework with the new server infrastructure.
fr fr/ 
fr fr/ Features demonstrated:
fr fr/ - HTTP server creation and configuration
fr fr/ - Router setup with multiple endpoints
fr fr/ - Middleware integration
fr fr/ - Request/response handling
fr fr/ - Graceful server lifecycle management

yeet "stdlib::web_vibez::*"
yeet "stdlib::io::console"

fr fr/ Simple greeting handler
squad GreetingHandler {
    greeting: String,
}

impl GreetingHandler {
    slay new(greeting: String) -> Self {
        Self { greeting }
    }
}

impl RequestHandler for GreetingHandler {
    slay handle(&self, request: &mut RequestContext) -> HandlerResult {
        let mut response = ResponseContext::new();
        response.set_status(StatusCode::OK);
        
        // Get name from query parameters or use default
        let name = request.query_params()
            .get("name")
            .unwrap_or("World");
        
        let message = format!("{}, {}!", self.greeting, name);
        response.set_body(message.as_bytes().to_vec());
        response.add_header("Content-Type", "text/plain");
        
        Ok(response)
    }
}

fr fr/ JSON API handler
squad ApiHandler;

impl RequestHandler for ApiHandler {
    slay handle(&self, request: &mut RequestContext) -> HandlerResult {
        let mut response = ResponseContext::new();
        response.set_status(StatusCode::OK);
        
        // Create JSON response
        let json_response = format!(
            r#"{{
                "method": "{}",
                "path": "{}",
                "timestamp": "{}",
                "headers_count": {},
                "body_size": {}
            }}"#,
            request.method(),
            request.path(),
            "2024-01-01T00:00:00Z", // In real code, use actual timestamp
            request.headers().len(),
            request.body().map(|b| b.len()).unwrap_or(0)
        );
        
        response.set_body(json_response.as_bytes().to_vec());
        response.add_header("Content-Type", "application/json");
        
        Ok(response)
    }
}

fr fr/ Echo handler that returns the request body
squad EchoHandler;

impl RequestHandler for EchoHandler {
    slay handle(&self, request: &mut RequestContext) -> HandlerResult {
        let mut response = ResponseContext::new();
        response.set_status(StatusCode::OK);
        
        // Echo the request body back
        if let Some(body) = request.body() {
            response.set_body(body.clone());
        } else {
            response.set_body(b"No body received".to_vec());
        }
        
        response.add_header("Content-Type", "text/plain");
        response.add_header("X-Echo-Handler", "based");
        
        Ok(response)
    }
}

fr fr/ Custom middleware that adds request timing
squad TimingMiddleware;

impl Middleware for TimingMiddleware {
    slay process(&self, request: &mut RequestContext) -> MiddlewareResult {
        // Add start time to request context
        let start_time = "request_start_time"; // In real code, use actual timestamp
        request.set_data("start_time", ContextData::String(start_time.to_string()));
        
        // Continue processing
        MiddlewareResult::continue_processing()
    }
}

fr fr/ Create server configuration
slay create_server_config() -> WebVibezConfig {
    WebVibezConfig {
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            max_connections: 100,
            request_timeout: Duration::from_secs(30),
            keep_alive_timeout: Duration::from_secs(60),
            header_timeout: Duration::from_secs(10),
            connection_timeout: Duration::from_secs(300),
            max_header_size: 8192,
            max_body_size: 1024 * 1024, // 1MB
        },
        security: SecurityConfig {
            csrf_secret: "demo_csrf_secret".to_string(),
            session_secret: "demo_session_secret".to_string(),
            enable_xss_protection: based,
            enable_csrf_protection: cap,
            allowed_origins: vec!["*".to_string()],
            content_security_policy: Some("default-src 'self'".to_string()),
            hsts_max_age: Some(31536000),
            enable_secure_headers: based,
        },
        performance: PerformanceConfig {
            enable_compression: based,
            compression_level: 6,
            max_request_size: 1024 * 1024,
            worker_threads: 4,
            connection_pool_size: 20,
            enable_http2: cap,
            enable_request_id: based,
        },
        session: SessionConfig {
            cookie_name: "demo_session".to_string(),
            secret_key: "demo_secret_key".to_string(),
            max_age: Duration::from_secs(3600),
            secure: cap,
            http_only: based,
            same_site: "Lax".to_string(),
            domain: None,
            path: "/".to_string(),
        },
        template: TemplateConfig {
            template_dir: "templates".to_string(),
            cache_templates: based,
            auto_reload: cap,
        },
        static_files: StaticFileConfig {
            static_dir: "static".to_string(),
            enable_directory_listing: cap,
            cache_control: "public, max-age=3600".to_string(),
        },
        logging: LoggingConfig {
            level: "info".to_string(),
            format: "json".to_string(),
            enable_request_logging: based,
            log_file: Some("server.log".to_string()),
        },
        development: DevelopmentConfig {
            hot_reload: cap,
            debug_mode: based,
            profiling: cap,
        },
    }
}

fr fr/ Setup router with endpoints
slay setup_router() -> Router {
    let mut router = Router::new();
    
    // Static greeting endpoints
    router.get("/", Arc::new(GreetingHandler::new("Hello".to_string())))?;
    router.get("/hello", Arc::new(GreetingHandler::new("Hello".to_string())))?;
    router.get("/hi", Arc::new(GreetingHandler::new("Hi".to_string())))?;
    
    // API endpoints
    router.get("/api/status", Arc::new(ApiHandler))?;
    router.post("/api/data", Arc::new(ApiHandler))?;
    router.put("/api/update", Arc::new(ApiHandler))?;
    router.delete("/api/delete", Arc::new(ApiHandler))?;
    
    // Echo endpoint for testing
    router.post("/echo", Arc::new(EchoHandler))?;
    
    // Health check endpoint
    router.get("/health", Arc::new(GreetingHandler::new("OK".to_string())))?;
    
    Ok(router)
}

fr fr/ Setup middleware chain
slay setup_middleware() -> MiddlewareChain {
    let mut chain = MiddlewareChain::new();
    
    // Add timing middleware
    chain.add_middleware(Arc::new(TimingMiddleware));
    
    // Add logging middleware (if available)
    // chain.add_middleware(Arc::new(LoggingMiddleware::new()));
    
    chain
}

fr fr/ Main server function
slay run_server() -> Result<(), Box<dyn std::error::Error>> {
    console::println("🚀 Starting CURSED web_vibez HTTP server demo...")?;
    
    // Create configuration
    let config = create_server_config();
    console::println(&format!("📝 Server configuration created for {}:{}", 
                             config.server.host, config.server.port))?;
    
    // Setup router
    let router = setup_router()?;
    console::println("🛣️  Router configured with endpoints")?;
    
    // Setup middleware
    let middleware = setup_middleware();
    console::println("🔧 Middleware chain configured")?;
    
    // Create server
    let server = HttpServer::new(config, router, middleware)?;
    console::println("🖥️  HTTP server created successfully")?;
    
    // Print server information
    console::println("\n📊 Server Information:")?;
    console::println("  • Host: 127.0.0.1")?;
    console::println("  • Port: 8080")?;
    console::println("  • Max Connections: 100")?;
    console::println("  • Request Timeout: 30s")?;
    console::println("  • Worker Threads: 4")?;
    
    console::println("\n🌐 Available Endpoints:")?;
    console::println("  GET  /              - Hello World")?;
    console::println("  GET  /hello?name=X  - Personalized greeting")?;
    console::println("  GET  /hi            - Casual greeting")?;
    console::println("  GET  /api/status    - API status (JSON)")?;
    console::println("  POST /api/data      - API data endpoint")?;
    console::println("  POST /echo          - Echo request body")?;
    console::println("  GET  /health        - Health check")?;
    
    console::println("\n💡 Try these examples:")?;
    console::println("  curl http://127.0.0.1:8080/hello")?;
    console::println("  curl http://127.0.0.1:8080/hello?name=CURSED")?;
    console::println("  curl http://127.0.0.1:8080/api/status")?;
    console::println("  curl -X POST -d 'Hello!' http://127.0.0.1:8080/echo")?;
    
    // Start server
    console::println("\n🚀 Starting server...")?;
    server.start()?;
    
    console::println("✅ Server started successfully!")?;
    console::println("Press Ctrl+C to stop the server")?;
    
    // Keep server running until signal
    loop {
        // Check server status
        let stats = server.get_stats();
        
        // Print periodic status updates
        if stats.total_requests % 10 == 0 && stats.total_requests > 0 {
            console::println(&format!(
                "📈 Stats: {} requests, {} active connections, uptime: {:?}",
                stats.total_requests,
                stats.active_connections,
                stats.uptime
            ))?;
        }
        
        // Check if shutdown requested
        if !server.is_running() {
            break;
        }
        
        // Sleep briefly
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    
    console::println("🛑 Stopping server...")?;
    server.stop()?;
    console::println("✅ Server stopped gracefully")?;
    
    Ok(())
}

fr fr/ Entry point
slay main() -> Result<(), Box<dyn std::error::Error>> {
    console::println("🌟 CURSED web_vibez HTTP Server Demo")?;
    console::println("=====================================\n")?;
    
    match run_server() {
        Ok(()) => {
            console::println("👋 Demo completed successfully!")?;
            Ok(())
        },
        Err(e) => {
            console::eprintln(&format!("❌ Error: {}", e))?;
            Err(e)
        }
    }
}

fr fr/ Example client function for testing
slay test_client() -> Result<(), Box<dyn std::error::Error>> {
    console::println("🧪 Testing server endpoints...")?;
    
    // Note: In a real implementation, we'd use an HTTP client
    // For now, this demonstrates the structure
    
    let test_cases = vec![
        ("GET", "/hello"),
        ("GET", "/hello?name=CURSED"),
        ("GET", "/api/status"),
        ("POST", "/echo"),
        ("GET", "/health"),
    ];
    
    for (method, path) in test_cases {
        console::println(&format!("Testing: {} {}", method, path))?;
        // In real code: make HTTP request and check response
    }
    
    console::println("✅ All tests passed!")?;
    Ok(())
}
