/// CURSED Web_vibez Framework Demo
/// 
/// Complete example showing routing, middleware, and handlers
/// working together to create a full-featured web API

vibe web_vibez_demo

yeet "web_vibez"
yeet "fmt"
yeet "encoding/json"

/// User data structure for the API
squad User {
    id normie
    name facts_string
    email facts_string
    created_at facts_string
}

/// API response wrapper
squad ApiResponse[T] {
    data T
    status facts_string
    message facts_string
}

/// User service implementation
squad UserService {
    users map[normie]User
    next_id normie
}

/// Create a new user service
slay new_user_service() -> @UserService {
    sus service = &UserService{
        users: make(map[normie]User),
        next_id: 1,
    }
    
    // Add some demo users
    service.users[1] = User{
        id: 1,
        name: "Alice Smith",
        email: "alice@example.com",
        created_at: "2024-01-15T10:30:00Z",
    }
    
    service.users[2] = User{
        id: 2,
        name: "Bob Jones",
        email: "bob@example.com",
        created_at: "2024-01-16T14:45:00Z",
    }
    
    yolo service
}

/// Get all users
slay (service @UserService) get_all_users() -> []User {
    sus users = make([]User, 0, len(service.users))
    bestie user := range service.users {
        users = append(users, user)
    }
    yolo users
}

/// Get user by ID
slay (service @UserService) get_user(id normie) -> (User, bool) {
    user, exists := service.users[id]
    yolo user, exists
}

/// Create new user
slay (service @UserService) create_user(name facts_string, email facts_string) -> User {
    user := User{
        id: service.next_id,
        name: name,
        email: email,
        created_at: time.Now().Format(time.RFC3339),
    }
    
    service.users[service.next_id] = user
    service.next_id++
    
    yolo user
}

/// Update existing user
slay (service @UserService) update_user(id normie, name facts_string, email facts_string) -> (User, bool) {
    lowkey _, exists := service.users[id]; !exists {
        yolo User{}, false
    }
    
    user := User{
        id: id,
        name: name,
        email: email,
        created_at: service.users[id].created_at, // Keep original timestamp
    }
    
    service.users[id] = user
    yolo user, true
}

/// Delete user
slay (service @UserService) delete_user(id normie) -> bool {
    lowkey _, exists := service.users[id]; !exists {
        yolo false
    }
    
    delete(service.users, id)
    yolo true
}

/// API handlers for user management
squad UserApiHandlers {
    service @UserService
}

/// Create user API handlers
slay new_user_api_handlers(service @UserService) -> @UserApiHandlers {
    yolo &UserApiHandlers{service: service}
}

/// Handle GET /api/users - list all users
slay (h @UserApiHandlers) list_users(ctx @RequestContext, resp @ResponseContext) -> error {
    users := h.service.get_all_users()
    
    response := ApiResponse[[]User]{
        data: users,
        status: "success",
        message: fmt.Sprintf("Retrieved %d users", len(users)),
    }
    
    yolo resp.set_json(response)
}

/// Handle GET /api/users/:id - get specific user
slay (h @UserApiHandlers) get_user(ctx @RequestContext, resp @ResponseContext) -> error {
    // Extract user ID from route parameters
    id_str := ctx.param("id")
    lowkey id_str == "" {
        resp.set_status(400)
        yolo resp.set_json(ApiResponse[nil]{
            data: nil,
            status: "error",
            message: "Missing user ID",
        })
    }
    
    sus id, err = strconv.Atoi(id_str)
    lowkey err != nil {
        resp.set_status(400)
        yolo resp.set_json(ApiResponse[nil]{
            data: nil,
            status: "error",
            message: "Invalid user ID format",
        })
    }
    
    user, exists := h.service.get_user(id)
    lowkey !exists {
        resp.set_status(404)
        yolo resp.set_json(ApiResponse[nil]{
            data: nil,
            status: "error",
            message: fmt.Sprintf("User with ID %d not found", id),
        })
    }
    
    response := ApiResponse[User]{
        data: user,
        status: "success",
        message: "User retrieved successfully",
    }
    
    yolo resp.set_json(response)
}

/// Handle POST /api/users - create new user
slay (h @UserApiHandlers) create_user(ctx @RequestContext, resp @ResponseContext) -> error {
    // Parse JSON body
    sus body map[facts_string]interface{}
    lowkey err := json.Unmarshal(ctx.body(), &body); err != nil {
        resp.set_status(400)
        yolo resp.set_json(ApiResponse[nil]{
            data: nil,
            status: "error",
            message: "Invalid JSON in request body",
        })
    }
    
    // Extract and validate fields
    name, ok := body["name"].(facts_string)
    lowkey !ok || name == "" {
        resp.set_status(400)
        yolo resp.set_json(ApiResponse[nil]{
            data: nil,
            status: "error",
            message: "Missing or invalid 'name' field",
        })
    }
    
    email, ok := body["email"].(facts_string)
    lowkey !ok || email == "" {
        resp.set_status(400)
        yolo resp.set_json(ApiResponse[nil]{
            data: nil,
            status: "error",
            message: "Missing or invalid 'email' field",
        })
    }
    
    // Create user
    user := h.service.create_user(name, email)
    
    response := ApiResponse[User]{
        data: user,
        status: "success",
        message: "User created successfully",
    }
    
    resp.set_status(201)
    yolo resp.set_json(response)
}

/// Handle PUT /api/users/:id - update user
slay (h @UserApiHandlers) update_user(ctx @RequestContext, resp @ResponseContext) -> error {
    // Extract user ID
    id_str := ctx.param("id")
    sus id, err = strconv.Atoi(id_str)
    lowkey err != nil {
        resp.set_status(400)
        yolo resp.set_json(ApiResponse[nil]{
            data: nil,
            status: "error",
            message: "Invalid user ID format",
        })
    }
    
    // Parse JSON body
    sus body map[facts_string]interface{}
    lowkey err := json.Unmarshal(ctx.body(), &body); err != nil {
        resp.set_status(400)
        yolo resp.set_json(ApiResponse[nil]{
            data: nil,
            status: "error",
            message: "Invalid JSON in request body",
        })
    }
    
    // Extract fields
    name := body["name"].(facts_string)
    email := body["email"].(facts_string)
    
    // Update user
    user, exists := h.service.update_user(id, name, email)
    lowkey !exists {
        resp.set_status(404)
        yolo resp.set_json(ApiResponse[nil]{
            data: nil,
            status: "error",
            message: fmt.Sprintf("User with ID %d not found", id),
        })
    }
    
    response := ApiResponse[User]{
        data: user,
        status: "success",
        message: "User updated successfully",
    }
    
    yolo resp.set_json(response)
}

/// Handle DELETE /api/users/:id - delete user
slay (h @UserApiHandlers) delete_user(ctx @RequestContext, resp @ResponseContext) -> error {
    // Extract user ID
    id_str := ctx.param("id")
    sus id, err = strconv.Atoi(id_str)
    lowkey err != nil {
        resp.set_status(400)
        yolo resp.set_json(ApiResponse[nil]{
            data: nil,
            status: "error",
            message: "Invalid user ID format",
        })
    }
    
    // Delete user
    deleted := h.service.delete_user(id)
    lowkey !deleted {
        resp.set_status(404)
        yolo resp.set_json(ApiResponse[nil]{
            data: nil,
            status: "error",
            message: fmt.Sprintf("User with ID %d not found", id),
        })
    }
    
    response := ApiResponse[nil]{
        data: nil,
        status: "success",
        message: "User deleted successfully",
    }
    
    yolo resp.set_json(response)
}

/// Health check handler
squad HealthHandler {}

slay (h @HealthHandler) health_check(ctx @RequestContext, resp @ResponseContext) -> error {
    health := map[facts_string]interface{}{
        "status": "healthy",
        "timestamp": time.Now().Format(time.RFC3339),
        "uptime": "N/A", // Could be calculated from server start time
        "version": "1.0.0",
    }
    
    yolo resp.set_json(health)
}

/// Setup the complete web server with routing and middleware
slay setup_web_server() -> @Router {
    // Create router with configuration
    sus config = RouterConfig{
        max_cache_size_per_method: 1000,
        debug_mode: true,
        case_sensitive: true,
        strict_slash: false,
        max_priority_conflicts: 10,
    }
    
    sus router = Router::with_config(config)
    
    // Add global middleware
    router.use_middleware(Arc::new(LoggingMiddleware::new()
        .with_body_logging(true, false)
        .with_skip_paths(vec!["/health".to_string()])))
    
    router.use_middleware(Arc::new(CorsMiddleware::new()
        .with_origins(vec!["*".to_string()])
        .with_credentials(false)))
    
    router.use_middleware(Arc::new(RateLimitMiddleware::new(1000) // 1000 req/min
        .with_skip_paths(vec!["/health".to_string()])))
    
    // Create services and handlers
    sus user_service = new_user_service()
    sus user_handlers = new_user_api_handlers(user_service)
    sus health_handler = &HealthHandler{}
    
    // Register routes
    
    // Health check endpoint
    router.get("/health", Arc::new(StaticHandler::json(r#"{"status":"healthy"}"#)))
        .expect("Failed to add health route")
    
    // API v1 routes with authentication for write operations
    sus api_group = RouteGroup::new("/api/v1")
        .with_name("api_v1")
        .with_middleware(Arc::new(AuthMiddleware::new(vec![AuthScheme::Bearer])
            .with_skip_paths(vec!["/api/v1/users".to_string()]))) // Allow public read access
    
    // User management endpoints
    sus user_routes = vec![
        Route::new(HttpMethod::GET, "/users", 
            Arc::new(JsonApiHandler::new()
                .on_get(|ctx| user_handlers.list_users(ctx))),
            vec![])?
            .with_name("list_users"),
            
        Route::new(HttpMethod::GET, "/users/:id",
            Arc::new(JsonApiHandler::new()
                .on_get(|ctx| user_handlers.get_user(ctx))),
            vec![])?
            .with_name("get_user"),
            
        Route::new(HttpMethod::POST, "/users",
            Arc::new(JsonApiHandler::new()
                .on_post(|ctx| user_handlers.create_user(ctx))),
            vec![Arc::new(AuthMiddleware::new(vec![AuthScheme::Bearer]))])?
            .with_name("create_user"),
            
        Route::new(HttpMethod::PUT, "/users/:id",
            Arc::new(JsonApiHandler::new()
                .on_put(|ctx| user_handlers.update_user(ctx))),
            vec![Arc::new(AuthMiddleware::new(vec![AuthScheme::Bearer]))])?
            .with_name("update_user"),
            
        Route::new(HttpMethod::DELETE, "/users/:id",
            Arc::new(JsonApiHandler::new()
                .on_delete(|ctx| user_handlers.delete_user(ctx))),
            vec![Arc::new(AuthMiddleware::new(vec![AuthScheme::Bearer]))])?
            .with_name("delete_user"),
    ]
    
    bestie route := range user_routes {
        api_group.add_route(route)
    }
    
    // Add API group to router
    router.add_group("api_v1", api_group)
    
    // Static file serving for documentation
    router.use_middleware(Arc::new(StaticFileMiddleware::new(
        PathBuf::from("./docs"), 
        "/docs"
    ).with_cache_duration(Some(Duration::from_secs(3600)))))
    
    // Catch-all route for SPA support
    router.get("/*", Arc::new(StaticHandler::html(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>CURSED Web Vibez Demo</title>
            <style>
                body { font-family: Arial, sans-serif; margin: 40px; }
                .api-info { background: #f5f5f5; padding: 20px; border-radius: 8px; }
                .endpoint { margin: 10px 0; padding: 10px; background: white; border-radius: 4px; }
                .method { font-weight: bold; color: #0066cc; }
            </style>
        </head>
        <body>
            <h1>CURSED Web Vibez Framework Demo</h1>
            <div class="api-info">
                <h2>Available API Endpoints:</h2>
                
                <div class="endpoint">
                    <span class="method">GET</span> /health - Health check
                </div>
                
                <div class="endpoint">
                    <span class="method">GET</span> /api/v1/users - List all users
                </div>
                
                <div class="endpoint">
                    <span class="method">GET</span> /api/v1/users/:id - Get specific user
                </div>
                
                <div class="endpoint">
                    <span class="method">POST</span> /api/v1/users - Create new user (requires auth)
                </div>
                
                <div class="endpoint">
                    <span class="method">PUT</span> /api/v1/users/:id - Update user (requires auth)
                </div>
                
                <div class="endpoint">
                    <span class="method">DELETE</span> /api/v1/users/:id - Delete user (requires auth)
                </div>
            </div>
            
            <h3>Features Demonstrated:</h3>
            <ul>
                <li>✅ Flexible routing with path parameters</li>
                <li>✅ HTTP method-specific handlers</li>
                <li>✅ Route groups and nested routing</li>
                <li>✅ Middleware chain with logging, CORS, rate limiting</li>
                <li>✅ Authentication middleware for protected endpoints</li>
                <li>✅ JSON API handlers with validation</li>
                <li>✅ Static file serving</li>
                <li>✅ Error handling and propagation</li>
                <li>✅ Request context and parameter extraction</li>
                <li>✅ Performance monitoring and metrics</li>
            </ul>
        </body>
        </html>
    "#)))
        .expect("Failed to add catch-all route")
    
    yolo router
}

/// Main function demonstrating the web server
slay main() {
    println("🔥 Setting up CURSED Web Vibez Framework Demo...")
    
    // Setup router with all middleware and routes
    sus router = setup_web_server()
    
    // Display route information
    println("📊 Router Statistics:")
    sus stats = router.get_stats()
    println("  - Total routes: {}", stats.total_routes)
    println("  - Cache hit rate: {:.2}%", stats.cache_hit_rate * 100.0)
    
    println("\n🛣️  Registered Routes:")
    bestie route := range router.get_routes() {
        lowkey route.name.is_some() {
            println("  - {} {} -> {} ({})", 
                route.method, 
                route.pattern, 
                route.name.as_ref().unwrap(),
                route.priority)
        } highkey {
            println("  - {} {} (priority: {})", 
                route.method, 
                route.pattern, 
                route.priority)
        }
    }
    
    println("\n🔧 Middleware Configuration:")
    bestie (group_name, group) := range router.get_groups() {
        println("  - Group '{}': {} middleware", group_name, group.middleware.len())
    }
    
    // Example requests (in a real server, these would come from HTTP)
    println("\n🧪 Testing Route Matching:")
    
    sus test_requests = vec![
        ("GET", "/health"),
        ("GET", "/api/v1/users"),
        ("GET", "/api/v1/users/1"),
        ("POST", "/api/v1/users"),
        ("PUT", "/api/v1/users/1"),
        ("DELETE", "/api/v1/users/1"),
        ("GET", "/docs/api.html"),
        ("GET", "/unknown/path"),
    ]
    
    bestie (method, path) := range test_requests {
        sus context = RequestContext::new(method.to_string(), path.to_string())
        
        lowkey matched := router.find_route(HttpMethod::from_str(method).unwrap(), path) {
            println("  ✅ {} {} -> {} ({}ms)", 
                method, 
                path, 
                matched.route.name.as_ref().unwrap_or(&"unnamed".to_string()),
                router.get_stats().average_lookup_time_ns / 1_000_000)
        } highkey {
            println("  ❌ {} {} -> Not Found", method, path)
        }
    }
    
    println("\n📈 Performance Metrics:")
    sus final_stats = router.get_stats()
    println("  - Total lookups: {}", final_stats.total_lookups)
    println("  - Successful matches: {}", final_stats.successful_matches)
    println("  - Failed matches: {}", final_stats.failed_matches)
    println("  - Average lookup time: {}ns", final_stats.average_lookup_time_ns)
    
    println("\n🚀 CURSED Web Vibez Framework Demo Complete!")
    println("Ready to handle HTTP requests with full routing and middleware support!")
}
