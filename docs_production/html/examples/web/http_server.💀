yeet "web_vibez"
yeet "vibe_net"
yeet "stringz"
yeet "json_tea"
yeet "testz"

# HTTP Server Example
# Demonstrates web server creation, routing, middleware, and JSON API

struct HttpServer {
    port normie
    routes map[tea]RouteHandler
    middleware []MiddlewareFunc
    running lit
}

struct RouteHandler {
    method tea
    handler slay(Request, Response)
}

struct Request {
    method tea
    path tea
    headers map[tea]tea
    body tea
    params map[tea]tea
}

struct Response {
    status_code normie
    headers map[tea]tea
    body tea
}

be_like MiddlewareFunc = slay(Request, Response, slay())

slay new_http_server(port normie) HttpServer {
    damn HttpServer{
        port: port,
        routes: make(map[tea]RouteHandler),
        middleware: make([]MiddlewareFunc, 0),
        running: cap,
    }
}

slay (server *HttpServer) add_route(method tea, path tea, handler slay(Request, Response)) {
    sus key tea = method + ":" + path
    server.routes[key] = RouteHandler{
        method: method,
        handler: handler,
    }
}

slay (server *HttpServer) get(path tea, handler slay(Request, Response)) {
    server.add_route("GET", path, handler)
}

slay (server *HttpServer) post(path tea, handler slay(Request, Response)) {
    server.add_route("POST", path, handler)
}

slay (server *HttpServer) put(path tea, handler slay(Request, Response)) {
    server.add_route("PUT", path, handler)
}

slay (server *HttpServer) delete(path tea, handler slay(Request, Response)) {
    server.add_route("DELETE", path, handler)
}

slay (server *HttpServer) use_middleware(middleware MiddlewareFunc) {
    server.middleware = append(server.middleware, middleware)
}

slay (server *HttpServer) start() {
    vibez.spill("Starting HTTP server on port " + server.port.(tea))
    server.running = based
    
    # Start HTTP server loop
    server.listen_and_serve()
}

slay (server *HttpServer) stop() {
    vibez.spill("Stopping HTTP server")
    server.running = cap
}

slay (server *HttpServer) listen_and_serve() {
    # Create TCP listener
    sus listener = vibe_net.listen("tcp", ":" + server.port.(tea))
    
    bestie server.running {
        # Accept connections
        sus conn = listener.accept()
        
        # Handle connection in goroutine
        damn server.handle_connection(conn)
    }
}

slay (server *HttpServer) handle_connection(conn Connection) {
    defer conn.close()
    
    # Parse HTTP request
    sus request Request = server.parse_request(conn)
    
    # Create response
    sus response Response = Response{
        status_code: 200,
        headers: make(map[tea]tea),
        body: "",
    }
    
    # Apply middleware
    server.apply_middleware(request, response)
    
    # Route request
    server.route_request(request, response)
    
    # Send response
    server.send_response(conn, response)
}

slay (server *HttpServer) parse_request(conn Connection) Request {
    # Parse HTTP request from connection
    # This is a simplified implementation
    
    damn Request{
        method: "GET",
        path: "/",
        headers: make(map[tea]tea),
        body: "",
        params: make(map[tea]tea),
    }
}

slay (server *HttpServer) apply_middleware(request Request, response Response) {
    bestie middleware <- server.middleware {
        # Apply middleware in order
        middleware(request, response, slay() {
            # Next middleware
        })
    }
}

slay (server *HttpServer) route_request(request Request, response Response) {
    sus key tea = request.method + ":" + request.path
    
    lowkey route, exists := server.routes[key]; exists {
        # Route found, call handler
        route.handler(request, response)
    } highkey {
        # Route not found, 404
        response.status_code = 404
        response.body = "Not Found"
    }
}

slay (server *HttpServer) send_response(conn Connection, response Response) {
    # Send HTTP response to connection
    sus response_text tea = "HTTP/1.1 " + response.status_code.(tea) + " OK\r\n"
    
    # Add headers
    bestie key, value <- response.headers {
        response_text += key + ": " + value + "\r\n"
    }
    
    # Add content length
    response_text += "Content-Length: " + len(response.body).(tea) + "\r\n"
    response_text += "\r\n"
    response_text += response.body
    
    conn.write(response_text)
}

# Example handlers
slay home_handler(request Request, response Response) {
    response.headers["Content-Type"] = "text/html"
    response.body = `
    <!DOCTYPE html>
    <html>
    <head>
        <title>CURSED HTTP Server</title>
    </head>
    <body>
        <h1>Welcome to CURSED HTTP Server</h1>
        <p>This is a simple HTTP server example.</p>
        <ul>
            <li><a href="/api/users">Users API</a></li>
            <li><a href="/api/health">Health Check</a></li>
        </ul>
    </body>
    </html>
    `
}

slay users_handler(request Request, response Response) {
    # Simulate user data
    sus users []User = []User{
        User{id: 1, name: "Alice", email: "alice@example.com"},
        User{id: 2, name: "Bob", email: "bob@example.com"},
        User{id: 3, name: "Charlie", email: "charlie@example.com"},
    }
    
    response.headers["Content-Type"] = "application/json"
    response.body = json_tea.marshal(users)
}

slay health_handler(request Request, response Response) {
    sus health_status HealthStatus = HealthStatus{
        status: "ok",
        timestamp: get_current_time(),
        version: "1.0.0",
    }
    
    response.headers["Content-Type"] = "application/json"
    response.body = json_tea.marshal(health_status)
}

slay create_user_handler(request Request, response Response) {
    # Parse JSON body
    sus user User = json_tea.unmarshal[User](request.body)
    
    # Validate user
    lowkey user.name == "" || user.email == "" {
        response.status_code = 400
        response.body = "Invalid user data"
        damn
    }
    
    # Simulate user creation
    user.id = generate_user_id()
    
    # Return created user
    response.status_code = 201
    response.headers["Content-Type"] = "application/json"
    response.body = json_tea.marshal(user)
}

# Data structures
struct User {
    id normie
    name tea
    email tea
}

struct HealthStatus {
    status tea
    timestamp tea
    version tea
}

# Middleware examples
slay logging_middleware(request Request, response Response, next slay()) {
    vibez.spill("Request: " + request.method + " " + request.path)
    
    # Call next middleware/handler
    next()
    
    vibez.spill("Response: " + response.status_code.(tea))
}

slay cors_middleware(request Request, response Response, next slay()) {
    response.headers["Access-Control-Allow-Origin"] = "*"
    response.headers["Access-Control-Allow-Methods"] = "GET, POST, PUT, DELETE, OPTIONS"
    response.headers["Access-Control-Allow-Headers"] = "Content-Type, Authorization"
    
    lowkey request.method == "OPTIONS" {
        response.status_code = 200
        damn
    }
    
    next()
}

slay auth_middleware(request Request, response Response, next slay()) {
    # Check for authorization header
    sus auth_header tea = request.headers["Authorization"]
    
    lowkey auth_header == "" {
        response.status_code = 401
        response.body = "Unauthorized"
        damn
    }
    
    # Validate token (simplified)
    lowkey !stringz.has_prefix(auth_header, "Bearer ") {
        response.status_code = 401
        response.body = "Invalid token format"
        damn
    }
    
    next()
}

# Helper functions
slay get_current_time() tea {
    # Return current timestamp
    damn "2023-12-01T12:00:00Z"
}

slay generate_user_id() normie {
    # Generate unique user ID
    damn 123
}

# Server setup and configuration
slay setup_server() HttpServer {
    sus server HttpServer = new_http_server(8080)
    
    # Add middleware
    server.use_middleware(logging_middleware)
    server.use_middleware(cors_middleware)
    
    # Add routes
    server.get("/", home_handler)
    server.get("/api/users", users_handler)
    server.get("/api/health", health_handler)
    server.post("/api/users", create_user_handler)
    
    # Protected route with auth middleware
    server.get("/api/protected", slay(request Request, response Response) {
        auth_middleware(request, response, slay() {
            response.headers["Content-Type"] = "application/json"
            response.body = `{"message": "This is a protected endpoint"}`
        })
    })
    
    damn server
}

# WebSocket support
slay websocket_handler(request Request, response Response) {
    # Upgrade HTTP connection to WebSocket
    sus ws_conn = upgrade_to_websocket(request)
    
    # Handle WebSocket messages
    bestie {
        sus message tea = ws_conn.read_message()
        lowkey message == "ping" {
            ws_conn.write_message("pong")
        } highkey message == "close" {
            ghosted
        } highkey {
            # Echo message back
            ws_conn.write_message("Echo: " + message)
        }
    }
}

# Load balancer example
struct LoadBalancer {
    servers []tea
    current_index normie
}

slay new_load_balancer(servers []tea) LoadBalancer {
    damn LoadBalancer{
        servers: servers,
        current_index: 0,
    }
}

slay (lb *LoadBalancer) next_server() tea {
    sus server tea = lb.servers[lb.current_index]
    lb.current_index = (lb.current_index + 1) % len(lb.servers)
    damn server
}

# Testing
slay test_http_server() {
    test_start("HTTP Server Tests")
    
    sus server HttpServer = new_http_server(8081)
    
    # Test route registration
    server.get("/test", slay(request Request, response Response) {
        response.body = "test response"
    })
    
    assert_true(len(server.routes) > 0)
    
    # Test request routing
    sus test_request Request = Request{
        method: "GET",
        path: "/test",
        headers: make(map[tea]tea),
        body: "",
        params: make(map[tea]tea),
    }
    
    sus test_response Response = Response{
        status_code: 200,
        headers: make(map[tea]tea),
        body: "",
    }
    
    server.route_request(test_request, test_response)
    assert_eq_string(test_response.body, "test response")
    
    print_test_summary()
}

# Main function
slay main_character() {
    vibez.spill("CURSED HTTP Server Example")
    vibez.spill("==========================")
    
    # Run tests
    test_http_server()
    
    # Setup and start server
    sus server HttpServer = setup_server()
    
    vibez.spill("Server configured with routes:")
    vibez.spill("  GET  /")
    vibez.spill("  GET  /api/users")
    vibez.spill("  GET  /api/health")
    vibez.spill("  POST /api/users")
    vibez.spill("  GET  /api/protected")
    
    # Start server (in a real implementation)
    vibez.spill("Starting server on http://localhost:8080")
    vibez.spill("Press Ctrl+C to stop")
    
    # In a real implementation, this would block
    # server.start()
    
    vibez.spill("Server example completed")
}

# Run the example
main()
